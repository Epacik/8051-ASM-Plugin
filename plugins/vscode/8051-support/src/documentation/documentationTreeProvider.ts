import * as vscode from "vscode" ;
import { LanguageClient } from "vscode-languageclient/node";
import IDocumentation from "./documentation";
import { nullishableString } from "../miscellaneousTypeAliases";
import DocumentationViewBase from "./documentationViewBase";
import IOpenDocsArguments from "./IOpenDocsArguments";

export class DocumentationTreeProvider extends DocumentationViewBase implements vscode.TreeDataProvider<TreeItem>  {

    #data: TreeItem[] | undefined;

    constructor(client: LanguageClient) {
        super(client);
    }
    onDidChangeTreeData?: vscode.Event<void | TreeItem | null | undefined> | undefined;

    getTreeItem(element: TreeItem): vscode.TreeItem|Thenable<vscode.TreeItem> {
    return element;
    }

    getChildren(element?: TreeItem|undefined): vscode.ProviderResult<TreeItem[]> {
    if (element === undefined) {
        if(this.#data === undefined) {
            return this.#refresh();
        }
        return this.#data;
    }
    return element.children;
    }

    async #refresh(): Promise<TreeItem[]> {
        let docs = await this.getDocumentation();

        let items: TreeItem[] = [];
        for (const [key, category, entries] of docs) {
            const children: TreeItem[] = [];

            for (const [name, _] of entries) {
                const child = new TreeItem(
                    name,
                    undefined,
                    new TreeCommand(name, `asm8051.openDocs`, {category: key, item: name})); 
                children.push(child);
            }

            const cat = new TreeItem(
                category, 
                children, 
                new TreeCommand(category, `asm8051.openDocs`, {category: key})); 
            items.push(cat);
        }

        this.#data = items;
        return items;
    }
}

class TreeItem extends vscode.TreeItem {
    children: TreeItem[] | undefined;
  
    constructor(label: string, children?: TreeItem[], command?: TreeCommand) {
      super(
          label,
          children === undefined ? vscode.TreeItemCollapsibleState.None :
                                   vscode.TreeItemCollapsibleState.Collapsed);
      this.children = children;
      this.command = command;
    }
}

class TreeCommand implements vscode.Command {

    constructor(title: string, command: string, arg?: IOpenDocsArguments) {
        this.title     = title;
        this.command   = command;
        this.arguments = [arg]
    }
    title: string;
    command: string;
    tooltip?: string | undefined;
    arguments?: any[] | undefined;

}