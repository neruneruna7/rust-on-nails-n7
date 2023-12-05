use crate::layout::Layout;
use db::User;
use dioxus::prelude::{dioxus_elements::tbody, *};

struct Props {
    users: Vec<User>,
}

// Vec<User> を取得し、HTML テーブルを作成します。
pub fn users(users: Vec<User>) -> String {
    // rsx を作成するための内部関数
    fn app(cx: Scope<Props>) -> Element {
        cx.render(rsx!(
            Layout {
                title: "Users Table",
                table {
                    thead {
                        tr {
                            th { "ID" }
                            th { "Email" }
                        }
                    }
                    tbody {
                        cx.props.users.iter().map(|user| rsx!(
                            tr {
                                td {
                                    strong { "{user.id}" }
                                }
                                td {
                                    "{user.email}"
                                }
                            }
                        ))
                    }
                }
                form {
                    action: "/sign_up",
                    method: "POST",
                    label {r#for: "user_email", "Email:" }
                    input {
                        id: "user_email",
                        name: "email",
                        r#type: "email",
                        required: "true",
                    }
                    button { "Submit" }
                }
            }

        ))
    }

    // コンポーネントを構築し、それを文字列にレンダリングします。
    let mut app = VirtualDom::new_with_props(app, Props { users });
    let _ = app.rebuild();
    dioxus::ssr::render_vdom(&app)
}
