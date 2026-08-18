---
title: "Introducing pgdmn: DMN in the database"
date: "2026-08-14T11:20:00-08:00"
description: >
  I made an open source PostgreSQL extension for executing business rules in PostgreSQL. Have business teams own and audit business rules, then execute them directly in your database without needing to translate them into code first.
---

I've just released [pgdmn](https://www.pgdmn.com), available now on github and soon on pgxn. You can read the larger introduction to DMN for developers and why [DMN in the database](https://www.pgdmn.com/articles/database-dmn/) on the pgdmn site, but I wanted to talk a little bit more about how I got to this project and some future directions for DMN in modern software development, such as within LLM-powered applications.

Btw, if you're looking to use pgdmn, or want to integrate DMN into your applications, including LLM-powered applications, give me an email at [fugu13@gmail.com](mailto:fugu13@gmail.com), I'd love to talk.

![Stylized example of DMN with BPMN](/kie-dmn-bpmn-integration.png)

## Appreciating business rules: Zelle fraud engine

A number of years ago I was part of a project implementing Zelle from scratch for a credit union. Zelle is a platform and a protocol for sending money person to person, instantly. While there is a central service handling coordination, individual banks and credit unions need to implement a complex set of services and integrations with their existing systems to support Zelle, especially as the institution assumes the risk for their side of Zelle transactions.

A critical element of any Zelle implementation is fraud detection, or more generally deciding whether to approve a transaction, reject it, or require further steps before approval, and we decided to make a standalone service.

One part of that service is less relevant here, a streaming data aggregator to provide real-time updated information on recent transactions and other customer activity. But the other part is very relevant: translating information on the current transaction, customer metadata, and real-time updated information into the decisions needed to decide how a transaction will be handled.

Early on we nearly went with "the decisions are code". But that was quickly reassessed. "If you want to change your fraud rules you need a developer in the loop" was not very satisfactory. Okay, we can provide dials for common changes to fraud rules (thresholds and so forth). "If you want to record the exact decision its basis for an audit we're going to need to do a lot more software development" was even less satisfactory.

We realized that what we were trying to build ad hoc, badly, at considerable development cost, already existed: executable, declarative decision models (aka rules).

For various reasons specific to the project and available technologies we ended up going with something other DMN, but we did use a declarative rules engine, and it dramatically simplified development while immediately providing strong answers to the customer's very reasonable requirements for basing fraud decisions on updatable, auditable rules owned by business teams.

I'd been aware of "rules engines" for quite some time, and used them occasionally, but this was the first time I did a deep dive on what modern business decision modeling looked like, and it has led me to a deep appreciation of the place for technologies that can, to a developer, look like they could be replaced by a handful of if statements.

## Making pgdmn

I've long loved Postgres. I like the choices Postgres has made about what a good database is like, and I think Postgres's emerging status as a database lingua franca is well-earned.

One of Postgres's greatest qualities is the extensions system, allowing database operators to include powerful new capabilities written by other people on top of a Postgres installation. Unsurprisingly, when I found myself reaching for DMN, which is for making decisions based on data, I checked if there was a extension to use DMN where I keep my data: in Postgres.

For several years I let that simmer. I've been writing increasing amounts of Rust, and I happened upon an excellent Rust implementation of DMN [DecisionToolkit (https://github.com/DecisionToolkit/dsntk)](https://github.com/DecisionToolkit/dsntk) during that time.

Several months ago I decided that if no one had made a Postgres extension for DMN, I would do so, and I would use it to explore some ideas about using Claude Code for development. I've spent a good bit of time iterating on pgdmn since then to get it exactly where I want it to be: a reliable and performant Postgres extension that makes it natural to work with DMN inside Postgres.

I've used the fantastic [pgrx (https://github.com/pgcentralfoundation/pgrx)](https://github.com/pgcentralfoundation/pgrx) framework for creating Postgres extensions in Rust to build pgdmn, using a vendored and slightly modified version of DecisionToolkit for core DMN logic, with extensive testing and guardrails to ensure DMN execution is safe and performant.

## What I hope people do with pgdmn

I think DMN should be used a lot more often by developers, and I think many of the barriers to adopting DMN come down when DMN is in the database. When DMN models are data and DMN evaluations are functions easily called in any query with data already in the database, using DMN to bridge between business decision development and software development is far more accessible.

Organizations already using DMN will find it immediately more useful: put your DMN library in the database, and then anyone familiar with SQL can quickly answer questions that might have been several rounds of back and forth with developers before, even if you haven't changed the actual application to use pgdmn yet.

Organizations newly encountering the constraints that make DMN useful, such as regulatory compliance, decision audit trails, or frequently updated business rules, have a new low-overhead option to incorporate DMN that doesn't require adopting an entirely new platform.

And I'm especially interested in one possible direction: DMN alongside LLMs.

## DMN with LLMs

I'll talk about this more in other posts, but I think DMN is a powerful tool in an LLM context. DMN models describe the data they need for executable decisions and the exact form the outputs of those decisions take, making them an excellent option as a pluggable "library" for LLMs to work from.

In sensitive applications, LLM steps can be used to marshal data that cannot be easily retrieved another way, before an orchestrator takes the marshaled data and feeds it into a DMN model owned by a business team that makes the final determination. LLMs provide the intake data flexibility, but do not get to make the decision, insulating the system from many potential risks.

## Next

I intend to extend pgdmn to provide safe DMN model evolution guards a la Kafka's Schema Registry, allowing for automated checks of, can we replace one model with another one without breaking the code evaluating the model?

I'm also working on DMN tooling for LLMs (MCP, Skills, etc). Not just evaluation, but deeper introspection and authoring constructs to make DMN manipulation natural in an LLM context. I'm very interested in talking to some folks who'd like to try these out, please reach out to me at fugu13@gmail.com if you'd like to have a conversation.
