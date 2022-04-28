<script lang="ts">
	import { app, event, http, invoke } from "@tauri-apps/api";
	import { Data } from "../store";
	import gameplayGif from "../../assets/outputm.gif";
	import { fade, fly } from "svelte/transition";
	import type { GithubTagApi, IInstallProgress } from "../DataTypes";
	import {
		getLatestLauncherTag,
		getLatestVulnusTag,
		getPercent,
		getTagDownload,
		getTagFromRef,
		installVersion,
		launchVulnus,
		removeVersion,
		versionInstalled,
	} from "../SharedFunctions";
	import { ChosenVersion, LatestVersionsAvailable, VersionsAvailable } from "../StoreData";
	import binIco from '../../assets/svg/binico.svg';
import { beforeUpdate, onDestroy, onMount } from "svelte";

	// let gamePictures = ["https://ichef.bbci.co.uk/news/976/cpsprodpb/16620/production/_91408619_55df76d5-2245-41c1-8031-07a4da3f313f.jpg"]
	// let showingPicture = 0;

	let showDropdown = false;
	// let chosenVersion = "v0.0.2";
	// let versions = $VersionsAvailable

	let versionSelector: HTMLElement;
	let addToDesktop = false
	$: isVersionInstalled = versionInstalled($ChosenVersion);
	function onFocusHandle(
		ev: MouseEvent & {
			currentTarget: EventTarget & HTMLElement;
			path?: HTMLElement[];
		}
	) {
		if (!ev.path.includes(versionSelector)) {
			showDropdown = false;
		}
	}
	//#region store.load
	event.once("client://store-loaded", async () => {
		console.log("store loaded../");
		let lastUpdate = Number(
			new Date(Data.Store.get.data.version.last_check as string).getTime() ?? 0
		);
		if (lastUpdate < Date.now() - 1e3 * 60 * 15) {
			await http.fetch<GithubTagApi.RootObject[]>(
				"https://api.github.com/repos/beat-game-dev/Vulnus/git/refs/tags"
			).then((rdata) => {
				// console.log("new err",rdata.data)
				// console.log(versions,rdata) //
				VersionsAvailable.set(
					rdata.data.map((v) => getTagFromRef(v.ref))
				);
				Data.Store.get.data.version.last_check = new Date(Date.now()).toISOString();
				Data.Store.get.data.version.versions = $VersionsAvailable
				// Data.Store.get.add(
				// 	"Vulnus.versions.last_check",
				// 	Date.now().toString()
				// );
				// Data.Store.get.add(
				// 	"Vulnus.versions",
				// 	JSON.stringify($VersionsAvailable)
				// );
			});
			await getLatestVulnusTag().then((tag) => {
				LatestVersionsAvailable.set(tag);
				console.log("latest",tag)
				// Data.Store.get.add("Vulnus.versions.latest", tag);
				Data.Store.get.data.version.latest = tag
				if (!$ChosenVersion) {
					ChosenVersion.set(tag)
					Data.Store.get.data.version.current = tag
					// Data.Store.get.add("Vulnus.versions.chosen", tag);
				}
			});
			await getLatestLauncherTag().then((tag) => {
				app.getVersion().then((ver) => {
					console.log("app ver", ver);
					if (tag.includes(ver)) {
						console.log("already updated");
					} else {
						event.emit("client://notification", {
							title: `Update Available`,
							data: `Go to my github to get the latest release!`,
						});
					}
				});
				Data.Store.get.data.launcher.latest = tag
				// Data.Store.get.add("Launcher.versions.latest", tag);
			});
		} else {
			console.log(
				`not updating cuz last update too soon (${lastUpdate})`
			);
		}
		Data.Store.get.write()
	});
	//#endregion

	let installingVersion = false;
	let installingText = "downloading"

	let installingTotal = 0;
	let installingProgress = 0;
	$: installingPercent = getPercent(installingTotal, installingProgress);


	let UnlistenToProgress : ReturnType<typeof event.listen>;
	onMount(()=>{
		// console.log("listen to progress")
		UnlistenToProgress = event.listen<IInstallProgress>("server://install-progress",evData=>{
			console.log(evData)
			if (evData.payload.state == "Done") {
				installingText = "extracting"
			} else if (evData.payload.state == "Start") {
				installingText = "downloading"
				installingProgress = evData.payload.current
				installingTotal = evData.payload.total
				installingVersion = true
			} else {
				installingProgress = evData.payload.current
				installingTotal = evData.payload.total
			}
		})

	})
	onDestroy(()=>{
		UnlistenToProgress.then(fn=>{
			fn()
			// console.log("unlisten to progress")
		})
	})

	function installVulnus() {
		installVersion($ChosenVersion,addToDesktop).then((_) => {
			installingVersion = false
			// ChosenVersion.set($ChosenVersion)
			isVersionInstalled = versionInstalled($ChosenVersion);
		});
	}
	function uninstallVulnus() {
		removeVersion($ChosenVersion).then((_) => {
			// ChosenVersion.set($ChosenVersion)
			isVersionInstalled = versionInstalled($ChosenVersion);
		});
	}
	function runVulnus() {
		console.log("launch");
		launchVulnus($ChosenVersion);
	}
	// save chosen version
	$:{
		if (Data.Store.get.data) {
			Data.Store.get.data.version.current = $ChosenVersion
			Data.Store.get.write()
		}
	}
</script>

<div class="flex w-full h-full flex-col">
	<!-- {@debug(chosenVersion)}	 -->
	<h1 class="text-gray-200 text-4xl">Vulnus Launcher</h1>
	<p class="text-gray-400 mb-2">Babaooey!</p>
	<!-- show game pictures -->
	<div class="relative flex-1 rounded-xl overflow-hidden">
		<div
			class="absolute vulnusBg w-full h-full rounded-xl overflow-hidden"
			style:background-image={`url(${gameplayGif})`}
		/>
		<div class="absolute h-full w-80 right-0 p-2 ">
			<div
				class="backdrop-blur-xl w-full h-full border-2 border-solid border-zinc-600/20 rounded-xl bg-zinc-900/40 flex flex-col items-center p-2"
			>
				<h2 class="text-gray-200 text-3xl mb-2">Select version</h2>
				<p class="text-gray-400">
					Install and run vulnus right from here!
				</p>
				<div class="mt-auto w-full">
					<div class="my-3">
						<label bind:this={versionSelector}>
							{#if showDropdown}
								<div
									transition:fly={{ y: 20 }}
									class="bg-zinc-900 mb-2 p-2 rounded-xl border border-solid border-zinc-600 shadow-xl"
								>
									{#each $VersionsAvailable as version}
										{@const isChosenVersion = version == $ChosenVersion}
										<!-- {@debug isChosenVersion,chosenVersion} -->
										<div>
											<button
												on:click={() => {
													ChosenVersion.set(version)
													// chosenVersion = version;
													console.log(
														"choose ",
														version
													);
													isVersionInstalled =
														versionInstalled(
															$ChosenVersion
														);
													showDropdown = false;
												}}
												class={`appearance-none min-w-full block select-none ${
													isChosenVersion
														? "text-gray-600"
														: "text-gray-400"
												}`}>{version}</button
											>
										</div>
									{/each}
									<!-- Latest -->
									<div>
										<button
											on:click={() => {
												ChosenVersion.set($LatestVersionsAvailable)
												// chosenVersion =
												// 	$LatestVersionsAvailable;
												console.log(
													"choose ",
													$ChosenVersion
												);
												isVersionInstalled =
													versionInstalled(
														$ChosenVersion
													);
												showDropdown = false;
											}}
											class={`appearance-none min-w-full block select-none ${
												$LatestVersionsAvailable ==
												$ChosenVersion
													? "text-gray-600"
													: "text-gray-400"
											}`}
											>{$LatestVersionsAvailable} (latest)</button
										>
									</div>
								</div>
							{/if}

							<input
								on:blur|preventDefault
								on:focus|preventDefault={(v) => {
									// if (v.) {

									// }
									showDropdown = true;
									// console.log("f",v)
								}}
								value={$ChosenVersion}
								on:beforeinput|preventDefault
								id="path"
								name="path"
								class={`appearance-none w-full text-neutral-200 placeholder-zinc-400 transition-colors bg-neutral-900 focus:outline-none focus:ring-pink-400 focus:border-pink-400 focus:ring-1 rounded-lg px-2 py-2 shadow-sm border`}
								placeholder="Path of vulnus"
							/>
						</label>
					</div>
					{#await isVersionInstalled}
						<p class="text-gray-400">loading.../</p>
					{:then installed}
						{#if installed}
							<div in:fade class="relative" >

								<div class="flex w-full flex-row space-x-3" >
									
									<button
										on:click={runVulnus}
										class="py-2 shadow-sm px-12 flex-1 transition-colors hover:bg-sky-600 text-gray-100 bg-sky-500 disabled:bg-sky-600/50 mt-2 rounded-lg"
										>Play!</button
									>
									<button
										on:click={uninstallVulnus}
										class="py-2 min-w-12 px-3 shadow-sm transition-colors hover:bg-rose-600 text-gray-100 bg-rose-500 disabled:bg-rose-600/50 mt-2 rounded-lg"
										><svelte:component class="h-4 w-4" this={binIco} /></button
									>
								</div>
							</div>
						{:else}
							<div in:fade class="my-3" >

								<!-- install settings -->
								<input bind:checked={addToDesktop} type="checkbox" id="dsktsc" class="form-check-input appearance-none h-4 w-4 border border-gray-300 rounded-sm bg-white checked:bg-pink-600 text-pink-600 checked:ring-pink-600 focus:ring-pink-600 checked:border-pink-600 focus:outline-none transition duration-200 mt-1 align-top bg-no-repeat bg-center bg-contain float-left mr-2 cursor-pointer " />
								<label for="dsktsc" class="text-gray-400 select-none" >
									Add shortcut to desktop?
								</label>

							</div>

							<button in:fade 
								on:click={installVulnus}
								class="py-2 w-full shadow-sm px-12 transition-colors hover:bg-emerald-600 text-gray-100 bg-emerald-500 disabled:bg-emerald-600/50 mt-2 rounded-lg"
								>Install</button
							>
							{#if installingVersion}

								<p for="downloading_vulnus" class="text-gray-400 select-none my-2" >
									{installingText}{ installingText == "downloading" ? `: ${installingPercent}` : ""}
								</p>
								<div class="w-full h-2 bg-zinc-800 rounded-lg relative overflow-hidden" >
									<div class="bg-pink-500 h-2" style:width={installingPercent} >

									</div>
								</div>
								<!-- <progress class="w-full rounded-lg" id="downloading_vulnus" value={installingProgress} max={installingTotal} /> -->
							{/if}
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
