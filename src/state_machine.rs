struct StartState {
    min_len: usize,
}

struct WaitingOnFooTxtState {
    min_len: usize,
    foo_txt_future: impl Future<Output = String>,
}

struct WaitingOnBarTxtState {
    content: String,
    bar_txt_future: impl Future<Output = String>,
}

struct EndState {}

enum ExampleStateMachine {
    Start(StartState),
    WaitingOnFooTxt(WaitingOnFooTxtState),
    WaitingOnBarTxt(WaitingOnBarTxtState),
    End(EndState),
}

impl Future for ExampleStateMachine {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        loop {
            match self {
                ExampleStateMachine::Start(state) => {
                    let foo_txt_future = async_read_file("foo.txt");
                    let state = WaitingOnFooTxtState {
                        min_len: state.min_len,
                        foo_txt_future,
                    };
                    *self = ExampleStateMachine::WaitingOnFooTxt(state);
                }
                ExampleStateMachine::WaitingOnFooTxt(state) => {
                    match state.foo_txt_future.poll(cx) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(content) => {
                            if content.len() < state.min_len {
                                let bar_txt_future = async_read_file("bar.txt");
                                let state = WaitingOnBarTxtState {
                                    content,
                                    bar_txt_future,
                                };
                                *self = ExampleStateMachine::WaitingOnBarTxt(state);
                            } else {
                                *self = ExampleStateMachine::End(EndState);
                                return Poll::Ready(content);
                            }
                        }
                    }
                }
                ExampleStateMachine::WaitingOnBarTxt(state) => {
                    match state..bar_txt_future.poll(cx) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(bar_txt) => {
                            *self = ExampleStateMachine::End(EndState);
                            return Poll::Ready(state.content + &bar_txt);
                        }
                    }
                }
                ExampleStateMachine::End(_) => {
                    panic!("poll called after Poll::Ready was returned");
                }
            }
        }
    }
}
