---
title: 'My path to create my own piece of the Web.'
date: 2024-07-27T20:44:24+02:00 
draft: false
tags: 
    - html
    - css 
    - javascript
    - ui 
    - ux 
    - rust 
    - axum 
    - bootstrap
    - markdown 
    - self-hosting     
    - backstory
---

There are many ways to create a website and the content for it. I experimented with some different ways to create a website, so I can say that the possibilities are near endless. 

## Prologue 
My first website ever was a Google Sites site with all my socials and things on it. It was bland, just like any Google Sites site. I also wrote a site for a High School project before that, also in Google Sites. Which made me like the top #1 person when it comes to Web in my High School (if you exclude the teachers) for a bit. 

## The bare HTML age
Then, I had to write a personal portfolio website for my Creative Technology degree. 
I wrote a bare HTML website, and it looked okay (for the skills I had). 
It had a funky animated background and even tried to work in mobile.
But the final conclusion was that I should not be left alone with styling a website. 
I even had a small JavaScript (script?) that fetched the articles one by one, because I expected to write so much that the website would take long to write otherwise. 

This was my first experience of writing any kind of front-end or UI, besides simple UI using the TkInter library for Python. 
This experience was very humbling. 

***HTML AND CSS CAN BE VERY HARD***

There are so many things you have to keep in mind. 
The first one is that the way of thinking for using the infamous HTML and CSS duo is way different. It takes a while to get used to.

The second one is that you probably want to make your website way flashier that you have the abilities to right now. The complexity of CSS code seems to increase in an exponential way the more elements, effects and quirks you want to add to your website. 

The third one is that there are many things that you should keep in mind when composing the website using HTML and styling the site using CSS, that you probably don't know of until you test it.
Remember the animated background? It was a image going back and forth. And when you let it play for very long, it did not move back properly anymore, so it was just gone.
There is also the question of accessibility, ever thought of (color)blind people using the computer? 

The fourth one that the web is a true place where all platforms and especially screen size thrive.
This means that the wide screen to use when creating your site, is not necessarily how your site is seen. 
A phone has different requirements for how the site looks, one of it is width of the site, and the another is the size of the text. 
Every screen and pixel is different. 
`1px` means something entirely different on each screen.  

## Bootstrap evolution
Because I did not know what I was doing for front-end,I moved to Bootstrap to replace CSS styling. 
This mad the site more consistent looks wise, and it no longer did wonky stuff on mobile. But it seemed too consistent, it looked like any other Bootstrap website due to how difficult it was to customize and how it relies on a lot of standard elements that are not really made to be customized. 

## Beyond static HTML
Another thing I tried to improve on my website is moving beyond static HTML. 
I started using Rust because of this. Yes I got tricked by the Rust fad. Don't take this as me hating on Rust. Rust is an amazing language. Just not for this. 

My first website with a backend was made using the Warp framework for Rust.
I apparently did not read the documentation.
Warp was not made for the shit I was trying to forge into. 
Next to warp I used the Tera templating engine which was a pretty good choice and I did not have that many issues with. This website did not go far as to implement what I wanted it to do, which is a reoccurring theme with this journey into backend development. 

My next move was a quick hopping between Axum and Actix, until I decided to use Axum. On the way I decided to switch from Tera to Askama, which was a more static approach and did stuff in a more Rusty way. On my way I managed to write some database integration for articles using [markdown](https://en.wikipedia.org/wiki/Markdown). Which I did not really use, because it was wonky and required me to write into the database manually.  

There was another iteration on the website using Axum along with SeaORM and HTMX to create a website where you could make the changes on the site itself. 
But I did not have time to finish it, and it fell to the wayside. 
I might revive it if I REALLY need to have a full site like that.

## Hugo: Return to Grug
My current website is made in Hugo, which is amazing but also very unremarkable. 
You write everything in Markdown, push the new content and changes to GitHub. And Hugo (along with my magical [GitHub action](https://github.com/Wilkuu-2/wilkuu.xyz/blob/main/.github/workflows/hugo.yml))
This is a way simpler way to do stuff and allows me to focus on writing. 

My way of writing those articles is using Obsidian, which I also use for journaling and notes, so I got the environment already dialed in for myself. But I might make a tutorial on the simplest way to claim your own piece of cyberspace. Nowadays, I just use Neovim for this. 


