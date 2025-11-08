---
title: 'A SSH Registration form for computer nerds'
date: 2025-09-30T16:25:31+02:00
draft: false
tags: 
    - SSH 
    - Rust 
    - TUI 
    - Ratatui
    - SNT
cover: '/memadd.png'
---
# Introduction 
Have you ever thought that signing in though a regular web-form was boring? {{< br >}}
Have you ever thought that ssh was the ultimate app hosting solution? {{< br >}} 
If you answered "Yes" to both questions you are in the right place. 

Introducing MemAdd (Member Add). 
An (as of now) half assed SSH app that lets people register for the amazing association that I am the secretary of (at the time of writing).

You can try it now using bash, or see the code on our [Gitlab repo](https://gitlab.snt.utwente.nl/jakub/memadd).
```bash
ssh signup.snt.utwente.nl
```

# Rationale
When I made this app, we were very short on members (and we still are), and did not have any official channel to sign up. 
Making a website seemed lame, and I was interested in TUI apps using [ratatui](https://ratatui.rs/) at the time, so I decided to cook something up. 

# The building process 

## SSH server in Rust
The first component needed would be the ssh server. 
You could _technically_ get `openssh` to do this for you by forcing a TUI app to appear and nothing else. This is doable, but technically insecure in vulnerable to spawning a bunch of processes due to the bots that regularly scan for SSH servers. 
Because of this, having the SSH server as part of the executable would become a requirement. 

That's where [russh](https://github.com/Eugeny/russh) comes in, a pretty lean ssh server library written in rust, just like `ratatui`. 

The `russh` library allows for a ton of things that SSH is capable of, but the most important thing is that you can receive `pty` requests. The `pty` request is a request for a terminal window with a given size and opens channels for input and output, and since the SSH client just passes the IO like a terminal in raw mode. 

This is perfect of use with `ratatui`

## Ratatui 
When it comes to ratatui, it's a straight forward TUI framework, that is backend-agnostic, as long as you make an implementation of the `Backend` trait, or bash the 
`Crossterm` backend into submission by putting a struct that is `Write` into it. 
Basically, you handle the IO yourself, and just draw into a buffer with this library. 

It got some nice capability for layout, since it is based on widgets that draw into the buffer based on size. 

### State 
Ratatui supports stateful widgets, which are widgets that take an additional argument when rendering that can be mutated while rendering. But this is not enough to maintain a whole app, you need a good scheme for different menus inside that state, and you need to change it depending on the user input. 

I solved this problem in a Rusty way, but implemented it in a way that could hurt the viewer's Rusticles. This was because I was figuring out all the Ratatui and Russh API's at the time, so at some point I had to make stuff async, then sync and then async again, so the whole state implementation looks bad. I will rectify it when I make a good russh backend for ratatui. 

## Example bashing 
To get ratatui and russh together, you need to somehow marry asynchronous(russh) and synchronous code (ratatui) together. 
Because I was (and still am) mediocre at Rust, I just used an [example](https://github.com/Eugeny/russh/blob/0ed1195025f4d2c72a35c241aaa29dfcd793276d/russh/examples/ratatui_app.rs) in the russh repo. 
Outside of making this link, one needs to orgainise the handling of the `pty`, channel `data` and `resize` events, which has been omitted.

```rust
// Snippet from MemAdd repo, adapted from the russh/examples/ratatui_app.rs example.
pub type SshTerm = Terminal<CrosstermBackend<SshHandler>>;

// Implemented like the russh example
pub struct SshHandler {
    sender: UnboundedSender<Vec<u8>>,
    buffer: Vec<u8>,
}

impl SshHandler {
    /// Starts a tokio task that diligently writes bytes sent through a channel.
    /// It returns the sending side (`SshHandler`)
    async fn start(handle: Handle, channel_id: ChannelId) -> Self {
        let (sender, mut receiver) = unbounded_channel::<Vec<u8>>();
        tokio::spawn(async move {
            while let Some(data) = receiver.recv().await {
                let result = handle.data(channel_id, data.into()).await;
                if result.is_err() {
                    trace!("Failed to send data: {:?}", result);
                }
            }
        });
        Self {
            sender,
            buffer: Vec::new(),
        }
    }
}

impl std::io::Write for SshHandler {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // Add to the buffer 
        self.buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // Throw the copy of the buffer over the channel
        let result = self.sender.send(self.buffer.clone());
        if let Err(e) = result {
            return Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, e));
        }
        // Then clear the buffer 
        self.buffer.clear();
        Ok(())
    }
}
...
```

## Input handling
You need to handle user input on your own in this case, since Crossterm only handles it in case of the app receiving it's input through `stdin` on the same terminal as the app is running.

I did not know yet that the amazing [Terminput](https://github.com/aschey/terminput) exists, so I just handled the raw input on my own by parsing the bytes into an Enum, very simple, but very bug prone and clunky. 

The next iteration of this app, that I'm planning to develop will use Terminput for sure. 

## Web API
Since it would be inconvenient to direct everyone to an SSH form, I also added a simple Axum API that takes the same arguments and does the same thing as submitting the form. 
Simple JSON POST route, that calls the submit function. Bless Serde for making this possible. 

## Submission 
I am not the best when it comes to creating secure code, and GDPR makes it, so you need to be one or not save any data at all. So, in a stroke of wisdom, I decided that the app will not store any data sent by the users on disk, rather, it will an email the board of the association upon submission. That way there is no saved data on the app and the board gets instantly notified. 

To do that I used [Lettre](https://github.com/lettre/lettre). 

### ICal in emails
The newest version of MemAdd implements the obscure method of inserting appointments into email by making the message multi-part and putting a base64 encoded ICal file into the email. It is not an attachment, it's another part of the message. I have no clue who thought this should be done this way, but we have it and now, you get an appointment for the upcoming lunch hour if you checked the correct option. 


