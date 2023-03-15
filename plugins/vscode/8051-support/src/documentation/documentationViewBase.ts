'use strict';
import { LanguageClient } from "vscode-languageclient/node";
import IDocumentation from "./documentation";
import { localize } from "vscode-nls-i18n";

export default class DocumentationViewBase {
    #client: LanguageClient;
    constructor(client: LanguageClient) {
        this.#client = client;
    }

    protected objectToMap(docs: any): Map<string, IDocumentation> {
        let result = new Map<string, IDocumentation>();

        for (const [key, value] of Object.entries(docs)) {
            result.set(key, <IDocumentation>value);
        }
        return result;
    }

    protected categorize(docs: ([name: string, data: IDocumentation])[]): Map<string, ([name: string, data: IDocumentation])[]> {
        let splittedDocs = new Map<string, ([name: string, data: IDocumentation])[]>();
        for (const [key, value] of docs) {
            if(!splittedDocs.has(value.category)) {
                splittedDocs.set(value.category, []);
            }

            splittedDocs.get(value.category)?.push([key, value]);
        }

        return splittedDocs;
    }

    protected sortDocs(docs: Map<string, IDocumentation>): ([name: string, data: IDocumentation])[]{
        const keys = Array.from(docs.keys());
        keys.sort();

        const result: ([name: string, data: IDocumentation])[] = [];

        for (const key of keys) {
            const item = docs.get(key);
            if(item === undefined) continue;

            result.push([key, item]);
        }

        return result;
    }

    protected sortCategories(
        docs: Map<string, ([name: string, data: IDocumentation])[]>,
        sortFunction?: (a: [key: string, label: string], b: [key: string, label: string]) => number)
        : ([key: string, label: string, entries: ([name: string, data: IDocumentation])[]])[] {
        
        if(sortFunction === undefined){
            sortFunction = (a, b): number => {
                if(a[1] < b[1]) return -1;
                else if (a[1] > b[1]) return 1;
                return 0;
            };
        }

        const keys: Array<[key: string, label: string]> = [];

        for (const key of docs.keys()){
            keys.push([key, localize('asm8051.views.documentationPanel.categories.' + key)]);
        }
        keys.sort();

        const sorted:([key: string, label: string, entries: ([name: string, data: IDocumentation])[]])[] = [];
        
        for (const [key, label] of keys) {
            const doc = docs.get(key);
            if(doc === undefined) continue;

            sorted.push([key, label, doc])
        }

        return sorted;
    }

    protected async getDocumentation(): Promise<([key: string, label: string, entries: ([name: string, data: IDocumentation])[]])[]> {
        const rawDocs = await this.#client.sendRequest("documentation/getAll")
        const docs: Map<string, IDocumentation> = this.objectToMap(rawDocs);
        const categorized = this.categorize(this.sortDocs(docs));
        return this.sortCategories(categorized);
    }
}