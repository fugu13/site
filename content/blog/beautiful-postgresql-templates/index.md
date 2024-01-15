---
title: 'Beautiful SQL Templates in PostgreSQL with PL/Rust and minijinja'
date: '2024-01-15T15:20:00-08:00'
description: >
  Going directly from SQL to structured text like HTML is really useful if you want to
  use frameworks like HTMX alongside backends like PostgREST, but databases don't have
  great text-formatting capabilities. Luckily, now that PL/Rust is available, Rust template frameworks
  like minijinja are available in PostgreSQL. This post shows how to use minijinja to return
  HTML directly from PostgreSQL.
---

Writing HTML-returning SQL isn't easy, because databases don't have good text formatting options. Using [PL/Rust](https://plrust.io),
you can use the [minijinja](https://github.com/mitsuhiko/minijinja) library, which is almost the same template syntax as the widely used Jinja2 and has
excellent error messages.

![a very simple HTML example](.perseus/static/html.jpg)

First, you'll need to connect with `psql` to a PostgreSQL instance supporting PL/Rust. One easy way to do that is
Omnigres, following the quick start at https://docs.omnigres.org/quick_start/. This post doesn't use any
features of Omnigres other than core PostgreSQL and PL/Rust. 

Once you're connected with `psql`, make a toy database to work with.

```sql
create table lists (id int, name text);

create table items (list_id int, item text, weight int);

insert into lists values (1, 'Important List');

insert into items values
    (1, 'Write SQL', 10),
    (1, 'Profit', 2),
    (1, '???', 8);
```

Now, how to make an HTML page representing an entire list? It isn't impossible, but it's awkward even with
this contrived example. You write an HTML escaping function, and use string_agg to
join together partial HTML for each list item, but this will rapidly become a hard to maintain mess.

Here's how to do it with PL/Rust and minijinja. First, set up the PL/Rust extension.

```sql
CREATE EXTENSION plrust;
```

Next, define a PL/Rust function to render the HTML.

```sql
CREATE OR REPLACE FUNCTION
    plrust.page(title TEXT, items TEXT[])
    RETURNS TEXT LANGUAGE plrust
AS
$$
[dependencies]
minijinja = "1.0.11"

[code]
use minijinja::render;

let rendered = render!(r#"
<html>
    <head>
        <title>{{ title }}</title>
    </head>
    <body>
        <h1>{{ title }}</h1>
        <ul>
        {% for item in items %}
            <li>{{ item }}</li>
        {% endfor %}
        </ul>
    </body>
</html>
"#,
    title => title.unwrap_or("No title"),
    items => items.unwrap().into_iter()
        .filter_map(|i| i)
        .collect::<Vec<_>>()
);

Ok(Some(rendered))

$$;
```

This function is compiled by PL/Rust when created. If your function has compilation errors, they'll
appear when you run the `CREATE FUNCTION` call, including all the high quality Rust error messages.
Additionally, you'll be able to see exactly how your Rust code is wrapped in a function signature
depending on the types of the arguments and the return type.

Other than slight transformation of the arguments to align with what the template needs,
they're slotted directly into the template.

Then, call your Rust function anywhere you want HTML from SQL.

```sql
select plrust.page(
    name,
    array_agg(item order by weight desc)
) from lists inner join items on lists.id = items.list_id group by lists.id, lists.name;
```

As your template setup grows, with nested templates and custom filters and so forth, you can move
your template infrastructure into a privately hosted crate and import it in your functions.

For another common scenario, maybe your SQL has JSON in it. You can work with JSON objects directly,
or you can use Rust's powerful serde capabilities to work with typed values.

```sql
create table json_stuff (data jsonb);

insert into json_stuff values (
    '{"title": "My Title", "items": ["yellow", "brick", "road"]}'::jsonb
);

CREATE OR REPLACE FUNCTION
    plrust.pagetwo(data JSONB)
    RETURNS TEXT LANGUAGE plrust
AS
$$
[dependencies]
minijinja = "1.0.11"
serde = "1.0"
serde_json = "1.0"

[code]
use minijinja::render;
use serde_json::from_value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct List {
    title: String,
    items: Vec<String>,
}

let data: List = serde_json::from_value(data.unwrap().0).unwrap();

let rendered = render!(r#"
<html>
    <head>
        <title>{{ data.title }}</title>
    </head>
    <body>
        <h1>{{ data.title }}</h1>
        <ul>
        {% for item in data.items %}
            <li>{{ item }}</li>
        {% endfor %}
        </ul>
    </body>
</html>
"#, data => data);

Ok(Some(rendered))

$$;

select plrust.pagetwo(data) from json_stuff;
```

The example here will error if the JSONB data structure from postgresql can't be loaded into the specified Rust
`struct`, but it is also possible to have fallback behavior--anything you can write in Rust.

While a small amount of Rust knowledge is needed for handling specific cases, my recommendation is to do almost
all the handling of individual values in SQL, then keep the Rust handling minimal. That way, no dedicated
Rust developer is needed. Instead, any developer can assemble what they need out of a small set of examples.

Note: PL/Rust can be configured to only allow specific dependencies. These examples would not work on AWS RDS
because PL/Rust on RDS does not allow most dependencies, including minijinja and serde.

