use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Lecture {
    pub id: u16,
    pub topic: String,
    pub notes: String,
}  

#[derive(Clone, Properties, PartialEq)]
pub struct LectureListProps {
    pub lectures: Vec<Lecture>,
    pub on_click: Callback<Lecture>
}

#[function_component(LectureList)]
pub fn lecture_list(LectureListProps { lectures, on_click }: &LectureListProps) -> Html {
    let on_click = on_click.clone();
    lectures.iter().map(|lecture| {
        let on_topic_select = {
            let on_click = on_click.clone();
            let lecture = lecture.clone();
            Callback::from(move |_| {
                on_click.emit(lecture.clone())
            })
        };
        html! {
            <h4 onclick={on_topic_select}>{format!("{}: {}", lecture.id, lecture.topic)}</h4>
        }
    }).collect()
}

#[derive(Clone, Properties, PartialEq)]
pub struct LectureDetailsProps {
    pub lecture: Lecture,
}

#[function_component(LectureDetails)]
pub fn lecture_details(LectureDetailsProps { lecture }: &LectureDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ lecture.topic.clone() }</h3>
            <p>{lecture.notes.clone()}</p>
        </div>
    }
}