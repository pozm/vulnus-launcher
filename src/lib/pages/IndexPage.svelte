<script lang="ts">
	import { app, event, http, invoke } from "@tauri-apps/api";
	import { Data } from "../store";
	import gameplayGif from "../../assets/outputm.gif";
	import { fly } from "svelte/transition";
	import type { GithubTagApi } from "../DataTypes";
	import {
		getLatestLauncherTag,
		getLatestVulnusTag,
		getTagDownload,
		getTagFromRef,
		installVersion,
		launchVulnus,
		versionInstalled,
	} from "../SharedFunctions";
	import { LatestVersionsAvailable, VersionsAvailable } from "../StoreData";

	// let gamePictures = ["https://ichef.bbci.co.uk/news/976/cpsprodpb/16620/production/_91408619_55df76d5-2245-41c1-8031-07a4da3f313f.jpg"]
	// let showingPicture = 0;

	let showDropdown = false;
	let chosenVersion = "v0.0.2";
	// let versions = $VersionsAvailable

	let versionSelector: HTMLElement;
	let addToDesktop = false
	let isVersionInstalled = versionInstalled(chosenVersion);
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

	event.once("client://store-loaded", () => {
		console.log("store loaded../");
		let lastUpdate = Number(
			Data.Store.get.entries["Vulnus.versions.last_check"] ?? 0
		);
		if (lastUpdate < Date.now() - 1e3 * 60 * 15) {
			http.fetch<GithubTagApi.RootObject[]>(
				"https://api.github.com/repos/beat-game-dev/Vulnus/git/refs/tags"
			).then((rdata) => {
				// console.log("new err",rdata.data)
				// console.log(versions,rdata)
				VersionsAvailable.set(
					rdata.data.map((v) => getTagFromRef(v.ref))
				);
				Data.Store.get.add(
					"Vulnus.versions.last_check",
					Date.now().toString()
				);
				Data.Store.get.add(
					"Vulnus.versions",
					JSON.stringify($VersionsAvailable)
				);
			});
			getLatestVulnusTag().then((tag) => {
				LatestVersionsAvailable.set(tag);
				Data.Store.get.add("Vulnus.versions.latest", tag);
			});
			getLatestLauncherTag().then((tag) => {
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
				Data.Store.get.add("Launcher.versions.latest", tag);
			});
		} else {
			console.log(
				`not updating cuz last update too soon (${lastUpdate})`
			);
		}
	});

	function installVulnus() {
		installVersion(chosenVersion,addToDesktop).then((_) => {
			isVersionInstalled = versionInstalled(chosenVersion);
		});
	}
	function runVulnus() {
		console.log("launch");
		launchVulnus(chosenVersion);
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
										<div>
											<button
												on:click={() => {
													chosenVersion = version;
													console.log(
														"choose ",
														version
													);
													isVersionInstalled =
														versionInstalled(
															chosenVersion
														);
													showDropdown = false;
												}}
												class={`appearance-none min-w-full block select-none ${
													version == chosenVersion
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
												chosenVersion =
													$LatestVersionsAvailable;
												console.log(
													"choose ",
													chosenVersion
												);
												isVersionInstalled =
													versionInstalled(
														chosenVersion
													);
												showDropdown = false;
											}}
											class={`appearance-none min-w-full block select-none ${
												$LatestVersionsAvailable ==
												chosenVersion
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
								value={chosenVersion}
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
							<button
								on:click={runVulnus}
								class="py-2 w-full shadow-sm px-12 transition-colors hover:bg-sky-600 text-gray-100 bg-sky-500 disabled:bg-sky-600/50 mt-2 rounded-lg"
								>Play!</button
							>
						{:else}
							<div class="my-3" >

								<!-- install settings -->
								<input bind:checked={addToDesktop} type="checkbox" id="dsktsc" class="form-check-input appearance-none h-4 w-4 border border-gray-300 rounded-sm bg-white checked:bg-pink-600 text-pink-600 checked:ring-pink-600 focus:ring-pink-600 checked:border-pink-600 focus:outline-none transition duration-200 mt-1 align-top bg-no-repeat bg-center bg-contain float-left mr-2 cursor-pointer " />
								<label for="dsktsc" class="text-gray-400 select-none" >
									Add shortcut to desktop?
								</label>

							</div>

							<button
								on:click={installVulnus}
								class="py-2 w-full shadow-sm px-12 transition-colors hover:bg-emerald-600 text-gray-100 bg-emerald-500 disabled:bg-emerald-600/50 mt-2 rounded-lg"
								>Install</button
							>
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
