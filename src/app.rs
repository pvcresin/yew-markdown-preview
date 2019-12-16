use pulldown_cmark::{html::push_html, Options, Parser};
use stdweb::web::Node;
use yew::virtual_dom::VNode;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct App {
    text: String,
}

pub enum Msg {
    Change(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {
            text: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Change(val) => {
                let mut options = Options::empty();
                options.insert(Options::ENABLE_TABLES);
                options.insert(Options::ENABLE_FOOTNOTES);
                options.insert(Options::ENABLE_STRIKETHROUGH);
                options.insert(Options::ENABLE_TASKLISTS);

                let parser = Parser::new_ext(&val, options);
                let mut parsed_text = String::new();
                push_html(&mut parsed_text, parser);
                self.text = parsed_text;
                true
            }
        }
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        let link = "https://github.com/pvcresin/yew-markdown-preview";
        let html_text = format!("<div class='markdown-body'>{}</div>", &self.text);
        let node = Node::from_html(&html_text).unwrap();
        let preview = VNode::VRef(node);
        html! {
            <>
                <header>
                    <p>
                        {"Yew Markdown Preview: "}
                        <a href={link}>{link}</a>
                    </p>
                </header>
                <div class={"container"}>
                    <textarea oninput=|e| Msg::Change(e.value) />
                    <div>{preview}</div>
                </div>
            </>
        }
    }
}
