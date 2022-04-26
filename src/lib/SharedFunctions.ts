import { event, invoke } from "@tauri-apps/api";
import { ShowInstallModal, VulnusPath } from "./StoreData";
import { get } from 'svelte/store';
import { Data } from "./store";
import { documentDir } from "@tauri-apps/api/path";
import { emit } from "@tauri-apps/api/event";
import {shell} from '@tauri-apps/api'
export function getTagDownload(tag:string) {
	return `https://github.com/beat-game-dev/Vulnus/releases/download/${tag}/Vulnus_Beta_Win.zip`
}
export function getTagFromRef(ref:string) {
	return ref.split('/')[2]
}
export function installVersion(tag,versionUrl:string) {
	event.emit("client://notification",{title:`Please wait`,data:`Installing version ${tag} may take a few seconds or minutes depending on your connection.`})
	return new Promise((res,rej)=>{

		documentDir().then(dir=>{
			let installPath = `${dir}vulnus-launcher/${tag}`
			console.log(installPath)
			invoke('install_vulnus',{vulnusDir:installPath,url:versionUrl}).then(_=>{
				console.log("OK")
				res()
				event.emit("client://notification",{title:`version ${tag} has been installed`,data:`Have fun hitting those notes`})
			},console.error)
		})
	})
}
export function versionInstalled(tag:string): Promise<boolean> {
	return new Promise((res,rej)=>{

		documentDir().then(dir=>{
			let installPath = `${dir}vulnus-launcher/${tag}`
			console.log(installPath)
			invoke<boolean>('check_vulnus_tag',{vulnusDir:installPath}).then(d=>{
				res(d)
			},console.error)
		})
	})
}
export function launchVulnus(tag:string) {
	documentDir().then(dir=>{
		let installPath = `${dir}vulnus-launcher/${tag}`
		shell.open(`${installPath}/Vulnus.exe`)
	})
}