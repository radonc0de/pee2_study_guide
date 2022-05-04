use yew::prelude::*;
mod lecture_list;
use lecture_list::LectureList;
use lecture_list::LectureDetails;
use lecture_list::Lecture;

#[function_component(App)]
fn app() -> Html {
    let lectures = vec![
        Lecture{
            id: 1, 
            topic: "Review of Circuit Elements".to_string(),
            notes: "Some words...".to_string(),
        },
        Lecture{
            id: 2,
            topic: "Some stuff".to_string(),
            notes: "more words...".to_string()
        },
        Lecture{
            id: 3,
            topic: "Some stuff".to_string(),
            notes: "more words...".to_string()
        }
    ];

    let selected_lecture = use_state(|| None);

    let on_lecture_select = {
        let selected_lecture = selected_lecture.clone();
        Callback::from(move |lecture: Lecture| {
            selected_lecture.set(Some(lecture))
        })
    };

    let details = selected_lecture.as_ref().map(|lecture| html! {
        <LectureDetails lecture={lecture.clone()} />
    });

    let details = selected_lecture.as_ref().map(|lecture| html! {
        <LectureDetails lecture={lecture.clone()} />
    });
    
    html! {
        <>
            <div>
                <h1>{ "Principles of Electrical Engineering 2 Study Guide" }</h1>
                <LectureList lectures={lectures} on_click={on_lecture_select.clone()} />
                {for details}
            </div>
            
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}