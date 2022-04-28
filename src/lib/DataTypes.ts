export interface IClientNotification {
	title:string
	data:string,
	html?:boolean
}
export interface IInstallProgress {
	total:number,
	current:number,
	state:string,
	tag:string
}

export interface IDataStoreTypes {
	version : IDataStoreVersion
	launcher: IDataStoreLauncher
}
export interface IDataStoreLauncher {
	latest : string
}
export interface IDataStoreVersion {
	current : string,
	latest : string,
	versions : string[],
	last_check : String
}

export module GithubTagApi {

    export interface Object {
        sha: string;
        type: string;
        url: string;
    }

    export interface RootObject {
        ref: string;
        node_id: string;
        url: string;
        object: Object;
    }

}

