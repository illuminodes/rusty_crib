use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct TitleProps {
    pub title: String,
}

#[function_component(SectionTitle)]
pub fn section_title(props: &TitleProps) -> Html {
    let class = r#"
        text-xl sm:text-2xl md:text-4xl font-bold 
        border-b-2 border-nostr-light 
        pb-4 mr-8 select-none"#;
    html! {
        <h2 {class}>
            {&props.title}
        </h2>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ParagraphProps {
    pub text: String,
}

#[function_component(SectionParagraph)]
pub fn section_paragraph(props: &ParagraphProps) -> Html {
    let class = r#"
        text-sm sm:text-base md:text-lg text-wrap select-none"#;
    html! {
        <p {class}>
            {&props.text}
        </p>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct SectionProps {
    pub children: Children,
}

#[function_component(AppSection)]
pub fn app_section(props: &SectionProps) -> Html {
    html! {
        <div class="flex flex-col gap-8 sm:gap-16">
            {props.children.clone()}
        </div>
    }
}

#[function_component(AppPage)]
pub fn app_page(props: &SectionProps) -> Html {
    html! {
        <div class="flex flex-col gap-16 sm:gap-24 sm:px-24 md:px-48 lg:px-72  justify-center">
            {props.children.clone()}
        </div>
    }
}
