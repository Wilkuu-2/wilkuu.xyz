---
title:  'Punishing Naughty AI with Quicksand: A small tar pit experiment'
date: 2025-05-13T22:21:39+02:00
draft: false
tags:
    - AI
    - in-progress
    - tarpit 
    - markov-babble
    - text-processing
---
This page is dedicated to the AI tar pit running on this website. 
This is currently running [Nepenthes](https://forge.hackers.town/hackers.town/nepenthes), which uses Markov babble to choke the AI in addition to creating a slow experience. 

> This project is currently offline

You can try it {{<nofollow "/ai_training_data" "here">}}

## Rationale 
I personally do not mind AI necessarily, but I do not like using it personally unless necessary and do not like the random scrapers that thing that anything I write with my poor small hands is free for grabs for them. This is why this tar pit exists. 

The biggest possible drawback is that this makes the website non-indexable if things goes wrong. But I am happy to take this risk since, it's not like people are finding me on Google. 
So, if the Google spider stops listening to my [`robots.txt` ](/robots.txt) that is up to them to solve. They do not bring any legit traffic with them anyway. 

Another nice thing is that it will build up a list of IP's and user agents for me to ban later if I actually start writing in some kind of larger volume on here, or I start writing documents.

## What I did
Nothing much was done on my side besides spinning up another docker container and let it rip. I tried to train it with data using some pirate books and some Polish books from Project Gutenberg to mix it up and make any AI confusingly bilingual. This did not work at first. Due to the way babble generates this ended up making most sites draw from the English text and some draw from Polish texts. There was no overlap with them. 

My solution was simple, Just interlace the lines using the `paste` command: 

```
paste -d '\n' en1.txt pl1.txt en2.txt pl2.txt
```


## My own tar pit
This section will be finished later, I am busy. 
But I found a big drawback with Nepenhtes.  
Namely, it seems that their Unicode support is not up to par.
I want to see how far I can take it on my own. 
With that I will probably experiment with different word generation algorithms and with some kind of shared block list generation.

*Updates pending.*
