
// https://gist.github.com/jamonserrano/b44e2ad9e4c90c722b9ebf981b9cb111
const document_ready = new Promise(resolve => {
    if (document.readyState === "complete") {
        resolve();
    } else {
        document.addEventListener("DOMContentLoaded", resolve);
    }
});

async function fetch_json(url) {
    let resp = await fetch(url);

    if (!resp.ok) {
        throw new Error(`Failed to fetch json '${url}'`);
    }

    let text = await resp.text();
    let json = JSON.parse(text);

    return json;
}

const render_last_updated = (selector, last_updated) => {
    document.querySelector(selector).innerText = last_updated;
}

const render_rfc_table = (selector, rfcs) => {
    let elem = document.querySelector(selector);
    elem.innerHTML = "";

    with (h) {
        elem.appendChild(
            table({}, [
                tr({}, [
                    th({}, [text("#")]),
                    th({}, [text("Name")]),
                    th({}, [text("Issues")]),
                ])
            ].concat(rfcs.map(rfc =>
                tr({}, [
                    td({}, [text(rfc.number)]),
                    td({}, [
                        a({href: `https://github.com/rust-lang/rfcs/blob/master/text/${rfc.file_name}`}, [
                            text(rfc.short_title),
                        ])
                    ]),
                    td({}, [h_issues(rfc)])
                ]))
            ))
        );
    }
}

const h_issues = (rfc) => {
    with (h) {
        return ul({class: "issues"}, rfc.issues.map(({url, owner, repo, number, state}) =>
            li({}, [
                a({class: `issue ${state}`, href: url}, [
                    div({}, [
                        text(`${owner}/${repo}#${number}`)
                    ])
                ])
            ])
        ))
    }
};

async function main() {
    let data = await fetch_json("data.json");

    await document_ready;

    console.log("hello?");

    render_last_updated(".last_updated", data.last_updated);
    render_rfc_table(".vbar.OPEN", data.open_rfcs);
    render_rfc_table(".vbar.UNKNOWN", data.unknown_rfcs);
    render_rfc_table(".vbar.CLOSED", data.closed_rfcs);
}

main()
