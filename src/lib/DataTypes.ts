export interface IClientNotification {
	title:string
	data:string,
	html?:boolean
	err?:boolean
}
export interface IInstallProgress {
	total:number,
	current:number,
	state:string,
	tag:string
}

export interface IDataStoreTypes {
	vulnus : IDataStoreVulnus
	launcher: IDataStoreLauncher
	modding : IDataStoreModding
}
export interface IDataStoreModding {
	mods : IModData[]
	source_list : string;
}
export interface IModData {
	available_versions:string[],
	current_version:string,
	name:string,
	download_url:string,
	last_updated:string,
	installed?:boolean,
}

export interface IDataStoreVulnus {
	version : IDataStoreVulnusVersion,
	path:string
}

export interface IDataStoreLauncher {
	latest_version : string
}
export interface IDataStoreVulnusVersion {
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

