# How to use

Run a loop until CTRL-C is pushed:
```
use ctrlc_handler::CtrlCHandler;

let handler = CtrlCHandler::new();
while handler.should_continue() {
   // Do stuff here
}
```
