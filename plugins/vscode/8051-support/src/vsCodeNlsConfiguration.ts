export default class VsCodeNlsConfiguration {
    readonly #language: string;
    readonly #languagePackSupport: boolean;
    
    public get language() : string {
        return this.#language;
    }

    public get languagePackSupport() : boolean {
        return this.#languagePackSupport;
    }
    
    constructor(language: string, languagePackSupport: boolean) {
        this.#language = language;
        this.#languagePackSupport = languagePackSupport;
    }

    public static getInstance = () => {
        if (this.#instance === null && process.env.VSCODE_NLS_CONFIG !== undefined) {
            const config = JSON.parse(process.env.VSCODE_NLS_CONFIG);
            this.#instance = new VsCodeNlsConfiguration(config.locale, config._languagePackSupport);
        }
        else if(this.#instance === null){
            this.#instance = new VsCodeNlsConfiguration("en", false);
        }
        return this.#instance;
    };
    static #instance: VsCodeNlsConfiguration | null = null;
}