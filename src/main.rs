use yew::prelude::*;
mod lecture_list;
use lecture_list::LectureList;
use lecture_list::LectureDetails;
use lecture_list::Lecture;

#[function_component(App)]
fn app() -> Html {
    let lectures = vec![
        Lecture{
            id: 0,
            topic: "Formula Sheet Ideas".to_string(),
            notes: "
                - P=IV, V=IR, all cap and ind equations \n
                - R and L add for series, C add for parallel \n
                -rc natural eqs
                -rc step eqs
                -general formula
                -rl nat and step
                -diode snubber how to
                ".to_string()
        },
        Lecture{
            id: 1, 
            topic: "Circuit Basics".to_string(),
            notes: String::from("Resistor: dissipates power, convert electrical E -> heat E, no memory
                Inductor: stores E in B field, short term current source, steady-state: short circuit
                Capppacitor: stores E in E field, short term voltage source, steady-sate: open circuit")
        },
        Lecture{
            id: 2,
            topic: "RC Natural Response".to_string(),
            notes: "
                Natural response: the response of a system to its internal E with no input \n
                Step response: response of a sys due to a sudden applied constant input \n
                Transient response: the response of a sys to a change from steady-state conditions \n
                RC Natural response: \n
                    vc(t) = Vo * e^(-t/RC) \n
                    ic(t) = Io * e^(-t/RC)
            ".to_string()
        },
        Lecture{
            id: 3,
            topic: "RC Step Response".to_string(),
            notes: "vc(t) =  V(1-e^(-t/RC)), ic(t) = (V/R)*e^(-t/RC)".to_string()
        },
        Lecture{
            id: 4,
            topic: "RL Natural + Step Response".to_string(),
            notes: "Natural: i(t) = (I0)*e^(-tR/L), 
                    Step: v(t) =  Ve^(-tR/L), i(t) = (V/R) * (1 - e^(-tR/L))".to_string()
        },
        Lecture{
            id: 5,
            topic: "Switching".to_string(),
            notes: "Diode Snubbers to protect your switch, also using multiple eqs with diff start values and time constant for diff stages of a circuit".to_string()
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