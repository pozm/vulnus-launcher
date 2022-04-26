<script lang="ts">
import { http, invoke } from "@tauri-apps/api";
import { Data } from "./store";
	import gameplayGif from '../assets/outputm.gif'
import { fly } from "svelte/transition";
import type {GithubTagApi} from './DataTypes'
import { getTagDownload, getTagFromRef, installVersion,launchVulnus,versionInstalled } from "./SharedFunctions";

	let gamePictures = ["https://ichef.bbci.co.uk/news/976/cpsprodpb/16620/production/_91408619_55df76d5-2245-41c1-8031-07a4da3f313f.jpg"]
	let showingPicture = 0;

	let showDropdown = false;
	let chosenVersion = "v0.0.2";
	let versions = ["v0.0.2", "v0.0.1"];

	let versionSelector:HTMLElement;

	let isVersionInstalled = versionInstalled(chosenVersion)

	function onFocusHandle(ev: MouseEvent & {
    currentTarget: EventTarget & HTMLElement;
	path?:HTMLElement[]
}) {
		if (!ev.path.includes(versionSelector)){
			showDropdown=false
		}
	}

	http.fetch<GithubTagApi.RootObject[]>("https://api.github.com/repos/beat-game-dev/Vulnus/git/refs/tags").then(rdata=>{
		versions = rdata.data.map(v=>getTagFromRef(v.ref))
		console.log(versions,rdata)
	})

	function installVulnus() {
		installVersion(chosenVersion).then(_=>{
			isVersionInstalled = versionInstalled(chosenVersion);
		})
	}
	function runVulnus() {
		console.log("launch")
		launchVulnus(chosenVersion)
	}

</script>

<div class="flex w-full h-full flex-col" >

	<!-- {@debug(chosenVersion)}	 -->
	<h1 class="text-gray-200 text-4xl" >Vulnus Launcher</h1>
	<p class="text-gray-400" >Babaooey!</p>
	<!-- show game pictures -->
	<div class="relative flex-1 rounded-xl overflow-hidden" >
		<div class="absolute vulnusBg w-full h-full rounded-xl overflow-hidden" style:background-image={`url(${gameplayGif})`} />
		<div class="absolute h-full w-80 right-0 p-2 " >
			<div class="backdrop-blur-xl w-full h-full border-2 border-solid border-zinc-600/20 rounded-xl bg-zinc-900/40 flex flex-col items-center p-2" >
				<h2 class="text-gray-200 text-3xl mb-2" >Select version</h2>
				<p class="text-gray-400" >
					Install and run vulnus right from here!
				</p>
				<div class="mt-auto w-full" >
					<div class="my-3">
						<label bind:this={versionSelector} >
							{#if showDropdown}
							<div transition:fly={{y:20}}
							class="bg-zinc-900 mb-2 p-2 rounded-xl border border-solid border-zinc-600 shadow-xl" >
								{#each versions as version}
									<div >
										<button on:click={()=>{
											chosenVersion = version;
											console.log("choose ",version)
											isVersionInstalled = versionInstalled(chosenVersion)
											showDropdown = false;
										}} class={`appearance-none min-w-full block select-none ${version == chosenVersion ? "text-gray-600" : "text-gray-400"}`} >{version}</button>
										<!-- <label on:change={()=>{
											showDropdown=false
										}} class={`min-w-full block select-none ${version == chosenVersion ? "text-gray-600" : "text-gray-400"}`} >
											<input class="appearance-none" type="radio" name={`version${version}`} value={version} bind:group={chosenVersion} />
											{version}
										</label> -->
									</div>
								{/each}
							</div>
						{/if}
							
							<input on:blur|preventDefault={(v)=>{
								console.log("b",v)
								// showDropdown=false
							}} on:focus|preventDefault={(v)=>{
								// if (v.) {

								// }
								showDropdown=true
								console.log("f",v)
							}} value={chosenVersion} on:beforeinput|preventDefault  id="path" name="path" class={`appearance-none w-full text-neutral-200 placeholder-zinc-400 transition-colors bg-neutral-900 focus:outline-none focus:ring-pink-400 focus:border-pink-400 focus:ring-1 rounded-lg px-2 py-2 shadow-sm border`} placeholder="Path of vulnus">
						</label>

					</div>
					{#await isVersionInstalled}
						<p class="text-gray-400" >loading.../</p>
					{:then installed} 
						{#if installed}
							<button on:click={runVulnus} class="py-2 w-full shadow-sm px-12 transition-colors hover:bg-sky-600 text-gray-100 bg-sky-500 disabled:bg-sky-600/50 mt-2 rounded-lg">Play!</button>

						{:else}
							<button on:click={installVulnus} class="py-2 w-full shadow-sm px-12 transition-colors hover:bg-emerald-600 text-gray-100 bg-emerald-500 disabled:bg-emerald-600/50 mt-2 rounded-lg">Install</button>

						{/if}
					{/await}
				</div>
			</div>
		</div>

	</div>
</div>
<svelte:body on:click={onFocusHandle} />
<style scoped>

	.vulnusBg {
		background-size: cover;
	}
	
</style>