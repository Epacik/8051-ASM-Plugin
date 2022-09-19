import IDocumentation from "../../documentation";
import { nullishableString } from "../../../miscellaneousTypeAliases";
import IOpenDocsArguments from "../../IOpenDocsArguments";

const isNullishOrWhitespace = (str: nullishableString) => str === null || str === undefined || str.trim() === "";

window.addEventListener('message', event => {
    const message = event.data; // The JSON data our extension sent
    const args = <IOpenDocsArguments>message;
    scrollToElement(args);
});

const scrollToElement = (args: IOpenDocsArguments) => {
    location.href = "#";
    let item: HTMLElement | null = null;
    if(!isNullishOrWhitespace(args.item)) {
        item = document.getElementById(args.item!);
    }
    else {
        item = document.getElementById(args.category);
    }

    if(item === null)
        return;
    
    item.scrollIntoView()

    //scroll by the size of headers on top of the page
    if(!isNullishOrWhitespace(args.item)){
        window.scrollBy(0, -60);
    }
    else {
        window.scrollBy(0, -35);
    }
}

if((<any>window).initialElement !== undefined){
    setTimeout(_ => scrollToElement(<IOpenDocsArguments>((<any>window).initialElement)), 500);
}
    

