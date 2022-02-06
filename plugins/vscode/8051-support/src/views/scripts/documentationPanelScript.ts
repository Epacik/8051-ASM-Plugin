import IDocumentation from "../../documentation";
import { nullishableString } from "../../miscellaneousTypeAliases";

window.addEventListener('message', event => {
    const message = event.data; // The JSON data our extension sent
    let docs = parseDocsData(message.docs);
    console.log("docs:");
    console.log(docs);

    let splittedDocs = splitDocsByCategory(docs);
    console.log("splitted docs");
    console.log(splittedDocs);

    const docsSection = document.getElementById("docs-list");
    if(docsSection !== null) {
        docsSection.innerHTML = "";
    }

    for (const [key, value] of splittedDocs) {
        docsSection?.insertAdjacentHTML("beforeend", `
        <h2 class="categoryHeader">
            <span>${key}</span>
            <vscode-divider role="separator"></vscode-divider>
        </h2>`);

        for (const [docsKey, doc] of value) {
            docsSection?.insertAdjacentHTML("beforeend", createDocElement(docsKey, doc));
        }

        docsSection?.insertAdjacentHTML("beforeend", `<vscode-divider role="separator"></vscode-divider>`);
    }
    
});

const parseDocsData = (json: string): Map<string, IDocumentation> => {
    const obj = JSON.parse(json);
    let docs = new Map<string, IDocumentation>();

    for (const [key, value] of Object.entries(obj)) {
        docs.set(key, <IDocumentation>value);
    }
    return docs;
}

const splitDocsByCategory = (docs: Map<string, IDocumentation>): Map<string, Map<string, IDocumentation>> => {
    let splittedDocs = new Map<string, Map<string, IDocumentation>>();
    for (const [key, value] of docs) {
        if(!splittedDocs.has(value.category)) {
            splittedDocs.set(value.category, new Map<string, IDocumentation>());
        }

        splittedDocs.get(value.category)?.set(key, value);
    }

    return splittedDocs;
}

function createDocElement(key: string, doc: IDocumentation): string {
    let result = `<h3>${key}</h3>`;
    if(!isNullishOrWhitespace(doc.detail)){
        result += `<h4>${doc.detail?.replace("\n", "<br/>")}</h4>`;
    }
    if(!isNullishOrWhitespace(doc.description)){
        result += `<p>${doc.description?.replace("\n", "<br/>")}</p>`;
    }
    if(!isNullishOrWhitespace(doc.syntax)){
        result += `<p>${doc.syntax?.replace("\n", "<br/>")}</p>`;
    }
    if(!isNullishOrWhitespace(doc.valid_operands)){
        result += `<p>${doc.valid_operands?.replace("\n", "<br/>")}</p>`;
    }
    if(!isNullishOrWhitespace(doc.affected_flags)){
        result += `<p>${doc.affected_flags?.replace("\n", "<br/>")}</p>`;
    }

    return result;
}

const isNullishOrWhitespace = (str: nullishableString) => str === null || str === undefined || str.trim() === "";