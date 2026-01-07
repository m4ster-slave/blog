---
title: "Reverse engineering driving school website"
date: "2026-01-7T10:00:00Z"
summary: "Quick writeup on how i reverse engineered my dirving school quiz website :P"
slug: "reverse-engineering-driving-school-website"
---

Let me clarify first that I don’t consider this skilled hacking or even something that gives me an advantage over others, since I still have to know how to drive in the end lol.

# Step 1: Analyzing the website

This little project started out how any web exploitation task does: in the Developer Tools of my browser. It was pretty obvious that the site fetches the questions from some kind of endpoint, but what I didn’t expect was that not only the questions but also the answers are preloaded.

![scheda endpoint](/assets/reverse-engineering-driving-school-website/scheda_endpoint.png)

After completing a quiz, a second endpoint gets called with an array of the answers.

# Step 2: Writing a study crutch

First, I wrote a small website that takes the JSON from the endpoint and displays the list of questions with their answers. But I wanted to go further. So I wrote a small browser extension that intercepts the traffic and displays the correct answer along with the explanation of why that answer is right (this is also included with the questions).

![extension](/assets/reverse-engineering-driving-school-website/extension.png)

After that, I looked at the endpoint that corrects and confirms the quiz. It looked really easy to create a “SOLVE ALL” button, so I did :).

# TL;DR

I made a browser extension that gives me the right answers or solves my driving school quizzes for me. I have no idea if this is legal since this is a state-owned website, but I’m just using a public endpoint and I’m not making tons of requests or anything.

I still need to do the test without this tool, so it’s probably wiser to actually do the quizzes now instead of writing stupid browser extensions :3.
