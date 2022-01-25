extern crate cursive;

// Creates the base layer of TUI
use cursive::Cursive;

// TextView holds the text for viewing on the Cursive layer.
use cursive::views::{
    Checkbox, Dialog, TextView, EditView, ListView
};

// For multi step TUI layers.
// for .with_id() and .call_on_id()
use cursive::traits::Identifiable;

// Keyboard event listener.
// use cursive::event::Key;

// Wrap all form fields values in one struct so we can
// pass around it easily.
struct CatsayOptions<'a> {
    message: &'a str,
    dead: bool
}

fn input_step(siv: &mut Cursive) {
    siv.pop_layer();
    siv.add_layer(
        Dialog::new()
            // Setting the title for the Dialog.
            .title("Please fill out the form for the cat.")
            // Sets the content inside the Dialog.
            .content(
                // Creates a list view in content.
                ListView::new()
                    // Sets child items inside the list.
                    .child(
                        "Message",
                        // Creates text edit field and sets a id for it.
                        EditView::new().with_id("message")
                    )
                    .child(
                        "Dead?",
                        // Creates a check box and sets a id for it.
                        Checkbox::new().with_id("dead")
                    )
            )
            // "Ok" Button for the dialog.
            // 's' is the Cursive passed into function.
            .button("Ok", |s|{
                // Gets the message from EditVied id and call on it.
                let message = s
                    // Calling on id and getting content via callback function.
                    // call_on_id() returns Option<T>. So unwraps it and assign value
                    // to message.
                    .call_on_id(
                        "message",
                        |t: &mut EditView| t.get_content()
                    ).unwrap();
                
                // Calling on id and getting content via callback function.
                let is_dead = s
                    // Same as above, but with Checkbox use is_checked()
                    // for boolen.
                    .call_on_id(
                        "dead",
                        |t: &mut Checkbox| t.is_checked()
                    ).unwrap();
                
                // Set values into struct.
                let options = CatsayOptions {
                    message: &message,
                    dead: is_dead
                };
                result_step(s, &options);
            })
            .button("Exit", |s| s.quit())
    );
}

fn result_step(siv: &mut Cursive, options: &CatsayOptions) {
    let eye = if options.dead {'x'} else {'o'};
    let cat_text = format!("
    {msg}
    \\
     \\
     /\\_/\\
    ( {eye} {eye} )
    =( I )=",
    msg = options.message,
    eye = eye
    );

    siv.pop_layer();
    siv.add_layer(
        Dialog::around(TextView::new(cat_text))
        .title("The cat says...")
        .button("Again", |s| input_step(s))
        .button("Ok", |s| s.quit())
    );
}

fn main() {
    let mut siv = Cursive::default();

    input_step(&mut siv);

    siv.run();   // Starting the event loop
}
