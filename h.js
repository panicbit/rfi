const create_element = (name, attrs, children) => {
    let node = document.createElement(name);

    for (let key of Object.keys(attrs)) {
        node.setAttribute(key, attrs[key]);
    }

    for (let child of children) {
        node.appendChild(child);
    }

    return node;
};

const tags = [
    'text', 'a', 'img', 'div', 'span', 'p',
    'h1', 'h2', 'h3', 'h4', 'h5', 'h6',
    'ul', 'ol', 'li',
    'table', 'tr', 'th', 'td',
];

window.h = (() => {
    let fns = {};

    for (let tag of tags) {
        if (tag === "text") {
            fns[tag] = document.createTextNode.bind(document);
        } else {
            fns[tag] = create_element.bind(this, tag);
        }
    }

    return fns;
})();
