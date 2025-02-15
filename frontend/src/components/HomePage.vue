<template>
	<div class="flex flex-col w-full h-full bg-green-50 max-w-full max-h-full overflow-hidden">
		<div v-if="settingsOpen" class="absolute z-10 w-full h-full flex justify-center items-center bg-black bg-opacity-25">
			<div class="bg-white rounded-xl shadow-xl p-4 w-[24rem]">
				<div class="flex flex-row justify-between items-center">
					<h3 class="font-medium text-xl">
						Settings
					</h3>
					<div class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out" @click="settingsOpen = false">
						Close
					</div>
				</div>
				<div class="py-4 flex flex-col">
					<div class="form-control hover:bg-gray-500 hover:bg-opacity-10 rounded-xl p-3">
						<label class="cursor-pointer flex flex-row justify-between items-center" @click="setAutostart(!autostart)">
							<span class="label-text">Start on boot</span>
							<input type="checkbox" :checked="autostart" class="checkbox focus:outline-none">
						</label>
					</div>
					<div class="form-control hover:bg-gray-500 hover:bg-opacity-10 rounded-xl p-3">
						<label class="cursor-pointer flex flex-row justify-between items-center" @click="setRealclose(!realclose)">
							<span class="label-text">Keep running on close</span>
							<input type="checkbox" :checked="!realclose" class="checkbox focus:outline-none">
						</label>
					</div>
				</div>
			</div>
		</div>

		<div class="flex flex-row justify-between items-center px-6 py-4">
			<!-- Header, Pc name left and options right -->
			<div>
				<h4 class="text-md">
					Device name
				</h4>
				<h2 class="text-2xl font-medium">
					{{ hostname }}
				</h2>
			</div>
			<div class="flex justify-center items-center gap-4">
				<p class="text-sm">
					v{{ version }}
				</p>
				<div class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out" @click="settingsOpen = true">
					<svg
						xmlns="http://www.w3.org/2000/svg" height="24"
						viewBox="0 -960 960 960" width="24">
						<!-- eslint-disable-next-line -->
						<path d="m370-80-16-128q-13-5-24.5-12T307-235l-119 50L78-375l103-78q-1-7-1-13.5v-27q0-6.5 1-13.5L78-585l110-190 119 50q11-8 23-15t24-12l16-128h220l16 128q13 5 24.5 12t22.5 15l119-50 110 190-103 78q1 7 1 13.5v27q0 6.5-2 13.5l103 78-110 190-118-50q-11 8-23 15t-24 12L590-80H370Zm70-80h79l14-106q31-8 57.5-23.5T639-327l99 41 39-68-86-65q5-14 7-29.5t2-31.5q0-16-2-31.5t-7-29.5l86-65-39-68-99 42q-22-23-48.5-38.5T533-694l-13-106h-79l-14 106q-31 8-57.5 23.5T321-633l-99-41-39 68 86 64q-5 15-7 30t-2 32q0 16 2 31t7 30l-86 65 39 68 99-42q22 23 48.5 38.5T427-266l13 106Zm42-180q58 0 99-41t41-99q0-58-41-99t-99-41q-59 0-99.5 41T342-480q0 58 40.5 99t99.5 41Zm-2-140Z"/>
					</svg>
				</div>
			</div>
		</div>
		<div class="flex-1 flex flex-row">
			<!-- Content -->
			<!-- When uploading: left info about current file being uploaded -->
			<!-- 				 right, list of nearby devices -->
			<!-- Default: left settings about visibility -->
			<!-- 		  right, ready to receive with hint for drag & drop, then request (to accept or not) -->
			<div class="w-72 p-3" v-if="outboundPayload === undefined">
				<p class="mt-4 mb-2 pt-3 px-3">
					Visibility state
				</p>
				<h4
					tabindex="0" role="button" class="btn font-medium flex flex-row !justify-between w-full
					items-center rounded-xl active:scale-95 transition duration-150 ease-in-out p-3" @click="invertVisibility()">
					<span v-if="visibility === 'Visible'">Always visible</span>
					<span v-else-if="visibility === 'Invisible'">Hidden from everyone</span>
					<span v-else>Temporarily visible</span>

					<svg
						xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"
						:class="{'rotate-180': visibility === 'Invisible'}">
						<path d="M504-480 320-664l56-56 240 240-240 240-56-56 184-184Z" />
					</svg>
				</h4>
				<p class="text-xs mt-2 pb-3 px-3">
					<span v-if="visibility === 'Visible'">
						Nearby devices can share files with you, but you'll always be
						notified and have to approve each transfer before receiving it.
					</span>
					<span v-else-if="visibility === 'Invisible'">
						No one can see your device at the moment. However, keep in mind that if another
						device has saved yours before, it might still attempt to start a transfer with you.
						<br>
						<br>
						You will get a notification when someone nearby is sharing
						giving you the ability to become visible for 1 minute.
					</span>
					<span v-else>
						You are temporarily visible to everyone.
					</span>
				</p>
			</div>
			<div class="w-72 p-6 flex flex-col justify-between" v-else>
				<div>
					<p class="mt-4 mb-2">
						Sharing {{ outboundPayload.Files.length }} file{{ outboundPayload.Files.length > 1 ? 's' : '' }}
					</p>
					<div class="bg-white w-32 h-32 rounded-2xl mb-2 flex justify-center items-center">
						<svg
							xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"
							class="w-8 h-8">
							<!-- eslint-disable-next-line -->
							<path d="M240-80q-33 0-56.5-23.5T160-160v-640q0-33 23.5-56.5T240-880h320l240 240v480q0 33-23.5 56.5T720-80H240Zm280-520v-200H240v640h480v-440H520ZM240-800v200-200 640-640Z" />
						</svg>
					</div>
					<p v-for="f in outboundPayload.Files" :key="f" class="overflow-hidden whitespace-nowrap text-ellipsis">
						{{ f.split('/').pop() }}
					</p>

					<p class="text-xs mt-3">
						Make sure both devices are unlocked, close together, and have bluetooth turned on. Device you're sharing with need
						Quick Share turned on and visible to you.
					</p>
				</div>

				<p
					@click="clearSending()"
					class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out w-fit">
					Cancel
				</p>
			</div>
			<div
				class="flex-1 grid grid-cols-1 bg-white w-full max-w-full min-w-0 min-h-full rounded-tl-[3rem] p-12 h-1 overflow-y-scroll"
				:class="{'grid-template-rows-auto': !displayedIsEmpty}">
				<h3 class="mb-4 font-medium text-xl">
					<span v-if="displayedIsEmpty">Ready to receive{{ outboundPayload != undefined ? ' / send' : '' }}</span>
					<span v-else>Nearby devices</span>
				</h3>

				<div v-if="displayedIsEmpty && endpointsInfo.length === 0" class="m-auto status-indicator status-indicator--success status-indicator--xl">
					<div class="circle circle--animated circle-main" />
					<div class="circle circle--animated circle-secondary" />
					<div class="circle circle--animated circle-tertiary" />
				</div>

				<div
					v-if="displayedIsEmpty && outboundPayload === undefined" class="w-full border
					rounded-2xl p-6 flex flex-col justify-center items-center transition duration-150 ease-in-out mt-auto"
					:class="{'border-green-200 bg-green-100 scale-105': isDragHovering}">
					<svg
						xmlns="http://www.w3.org/2000/svg" height="24"
						viewBox="0 -960 960 960" width="24" class="w-8 h-8">
						<!-- eslint-disable-next-line -->
						<path d="M440-320v-326L336-542l-56-58 200-200 200 200-56 58-104-104v326h-80ZM240-160q-33 0-56.5-23.5T160-240v-120h80v120h480v-120h80v120q0 33-23.5 56.5T720-160H240Z" />
					</svg>
					<h4 class="mt-2 font-medium">
						Drop files to send
					</h4>
					<div class="btn mt-2 active:scale-95 transition duration-150 ease-in-out" @click="openFilePicker()">
						<svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24">
							<path d="M440-440H200v-80h240v-240h80v240h240v80H520v240h-80v-240Z" />
						</svg>
						<span class="ml-2">Select</span>
					</div>
				</div>

				<div
					v-for="item in displayedItems" :key="item.id" class="w-full rounded-3xl flex flex-row gap-6 p-4 mb-4 bg-green-100"
					:class="{'cursor-pointer': item.endpoint}" @click="item.endpoint && sendInfo(item.id)">
					<div>
						<div class="h-16 w-16 rounded-full bg-white">
							<svg
								v-if="item.state === 'Finished'" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
								width="24" class="w-full h-full p-4">
								<!-- eslint-disable-next-line -->
								<path d="M268-240 42-466l57-56 170 170 56 56-57 56Zm226 0L268-466l56-57 170 170 368-368 56 57-424 424Zm0-226-57-56 198-198 57 56-198 198Z" />
							</svg>
							<svg
								v-else-if="item.deviceType === 'Laptop'" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
								width="24" class="w-full h-full p-4">
								<!-- eslint-disable-next-line -->
								<path d="M0-160v-80h160v-40q-33 0-56.5-23.5T80-360v-400q0-33 23.5-56.5T160-840h640q33 0 56.5 23.5T880-760v400q0 33-23.5 56.5T800-280v40h160v80H0Zm160-200h640v-400H160v400Zm0 0v-400 400Z" />
							</svg>
							<svg
								v-else-if="item.deviceType === 'Phone'" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
								width="24" class="w-full h-full p-4">
								<!-- eslint-disable-next-line -->
								<path d="M280-40q-33 0-56.5-23.5T200-120v-720q0-33 23.5-56.5T280-920h400q33 0 56.5 23.5T760-840v720q0 33-23.5 56.5T680-40H280Zm0-120v40h400v-40H280Zm0-80h400v-480H280v480Zm0-560h400v-40H280v40Zm0 0v-40 40Zm0 640v40-40Z" />
							</svg>
							<svg
								v-else-if="item.deviceType === 'Tablet'" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
								width="24" class="w-full h-full p-4">
								<!-- eslint-disable-next-line -->
								<path d="M120-160q-33 0-56.5-23.5T40-240v-480q0-33 23.5-56.5T120-800h720q33 0 56.5 23.5T920-720v480q0 33-23.5 56.5T840-160H120Zm40-560h-40v480h40v-480Zm80 480h480v-480H240v480Zm560-480v480h40v-480h-40Zm0 0h40-40Zm-640 0h-40 40Z" />
							</svg>
							<svg
								v-else xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
								width="24" class="w-full h-full p-4">
								<!-- eslint-disable-next-line -->
								<path d="M280-160H160q-33 0-56.5-23.5T80-240v-480q0-33 23.5-56.5T160-800h640v80H160v480h120v80Zm160-100q25 0 42.5-17.5T500-320q0-25-17.5-42.5T440-380q-25 0-42.5 17.5T380-320q0 25 17.5 42.5T440-260Zm-80 100v-71q-19-17-29.5-40T320-320q0-26 10.5-49t29.5-40v-71h160v71q19 17 29.5 40t10.5 49q0 26-10.5 49T520-231v71H360Zm480 0H640q-17 0-28.5-11.5T600-200v-360q0-17 11.5-28.5T640-600h200q17 0 28.5 11.5T880-560v360q0 17-11.5 28.5T840-160Zm-160-80h120v-280H680v280Zm0 0h120-120Z" />
							</svg>
						</div>

						<p
							v-if="(item.state === 'WaitingForUserConsent' || item.state === 'SentIntroduction') && item.pin_code"
							class="text-center inline-flex gap-1 mt-4 text-sm items-center">
							<svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24">
								<!-- eslint-disable-next-line -->
								<path d="M420-360h120l-23-129q20-10 31.5-29t11.5-42q0-33-23.5-56.5T480-640q-33 0-56.5 23.5T400-560q0 23 11.5 42t31.5 29l-23 129Zm60 280q-139-35-229.5-159.5T160-516v-244l320-120 320 120v244q0 152-90.5 276.5T480-80Zm0-84q104-33 172-132t68-220v-189l-240-90-240 90v189q0 121 68 220t172 132Zm0-316Z" />
							</svg>
							{{ item.pin_code }}
						</p>
					</div>
					<div class="flex-1 flex flex-col text-sm min-w-0" :class="{'justify-center': item.state === undefined}">
						<h4 class="text-base font-medium">
							{{ item.name }}
						</h4>

						<div v-if="item.state === 'WaitingForUserConsent'" class="flex-1 flex flex-col justify-between">
							<p class="mt-4">
								Wants to share {{ item.files?.join(', ') ?? item.text_description ?? 'some file(s).' }}
							</p>
							<div class="flex flex-row justify-end gap-4 mt-1">
								<p
									@click="sendCmd(item.id, 'AcceptTransfer')" class="btn px-3
									rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none">
									Accept
								</p>
								<p
									@click="sendCmd(item.id, 'RejectTransfer')" class="btn px-3
									rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none">
									Decline
								</p>
							</div>
						</div>

						<div v-else-if="['SentIntroduction', 'SendingFiles', 'ReceivingFiles'].includes(item.state ?? 'Initial')">
							<p class="mt-2" v-if="['SentIntroduction', 'SendingFiles'].includes(item.state ?? 'Initial')">
								Sending...
							</p>
							<p class="mt-2" v-else>
								Receiving...
							</p>
							<p v-for="f in item.files ?? []" :key="f" class="overflow-hidden whitespace-nowrap text-ellipsis">
								{{ f }}
							</p>
							<div class="flex flex-row justify-end gap-4 mt-1">
								<p
									@click="sendCmd(item.id, 'CancelTransfer')" class="btn px-3
									rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none">
									Cancel
								</p>
							</div>
						</div>

						<div v-else-if="item.state === 'Finished'">
							<p class="mt-2">
								Received <span v-if="!item.files && item.destination">link</span>
							</p>
							<p v-if="item.destination" :class="{'overflow-hidden whitespace-nowrap text-ellipsis': !item.files}">
								<span v-if="item.files">Saved to </span>{{ item.destination }}
							</p>
							<div class="flex flex-row justify-end gap-4 mt-1">
								<p
									v-if="item.destination" @click="invoke('open_url', { message: item.destination })"
									class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none">
									Open
								</p>
								<p
									@click="removeRequest(item.id)" class="btn px-3
									rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none">
									Clear
								</p>
							</div>
						</div>

						<div v-else-if="item.state === 'Cancelled'">
							<p class="mt-2">
								Transfer cancelled
							</p>
							<div class="flex flex-row justify-end gap-4 mt-1">
								<p
									@click="removeRequest(item.id)" class="btn px-3
									rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none">
									Clear
								</p>
							</div>
						</div>

						<div v-else-if="item.state === 'Rejected'">
							<p class="mt-2">
								Transfer rejected
							</p>
							<div class="flex flex-row justify-end gap-4 mt-1">
								<p
									@click="removeRequest(item.id)" class="btn px-3
									rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none">
									Clear
								</p>
							</div>
						</div>

						<div v-else-if="item.state === 'Disconnected'">
							<p class="mt-2">
								Unexpected disconnection
							</p>
							<div class="flex flex-row justify-end gap-4 mt-1">
								<p
									@click="removeRequest(item.id)" class="btn px-3
									rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none">
									Clear
								</p>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>

<script lang="ts">
import { ref, nextTick } from 'vue'
import { UnlistenFn, listen } from '@tauri-apps/api/event'
import { Store } from 'tauri-plugin-store-api';
import { invoke } from '@tauri-apps/api/tauri'
import { getVersion } from '@tauri-apps/api/app';
import { isPermissionGranted, requestPermission } from '@tauri-apps/api/notification';
import { getCurrent } from '@tauri-apps/api/window';
import { disable, enable } from 'tauri-plugin-autostart-api';

import { opt } from '../utils';
import { ChannelMessage } from '../../../core_lib/bindings/ChannelMessage';
import { ChannelAction } from '../../../core_lib/bindings/ChannelAction';
import { EndpointInfo } from '../../../core_lib/dist/EndpointInfo';
import { OutboundPayload } from '../../../core_lib/bindings/OutboundPayload';
import { SendInfo } from '../../../core_lib/bindings/SendInfo';
import { State } from '../../../core_lib/bindings/State';
import { DeviceType } from '../../../core_lib/bindings/DeviceType';
import { Visibility } from '../../../core_lib/bindings/Visibility';
import { dialog } from '@tauri-apps/api';

interface ToDelete {
	id: string,
	triggered: number
}

interface DisplayedItem {
	id: string,
	name: string,
	deviceType: DeviceType,
	endpoint: boolean,

	state?: State,
	pin_code?: string,
	files?: string[],
	text_description?: string,
	destination?: string,
}

const visibilityToNumber: { [key in Visibility]: number } = {
	'Visible': 0,
	'Invisible': 1,
	'Temporarily': 2,
};

const numberToVisibility: { [key: number]: Visibility } = {
	0: "Visible",
	1: "Invisible",
	2: "Temporarily",
};

const autostartKey = "autostart";
const realcloseKey = "realclose";
const visibilityKey = "visibility";
const stateToDisplay: Array<Partial<State>> = ["ReceivedPairedKeyResult", "WaitingForUserConsent", "ReceivingFiles", "Disconnected",
	"Finished", "SentIntroduction", "SendingFiles", "Cancelled", "Rejected"]

export default {
	name: "HomePage",

	setup() {
		const store = new Store(".settings.json");

		return {stateToDisplay, invoke, getVersion, store};
	},

	data() {
		return {
			isAppInForeground: false,
			discoveryRunning: ref(false),
			isDragHovering: ref(false),

			requests: ref<ChannelMessage[]>([]),
			endpointsInfo: ref<EndpointInfo[]>([]),
			toDelete: ref<ToDelete[]>([]),
			outboundPayload: ref<OutboundPayload | undefined>(),

			cleanupInterval: opt<NodeJS.Timeout>(),
			unlisten: Array<UnlistenFn>(),

			version: opt<string>(),

			autostart: ref<boolean>(true),
			realclose: ref<boolean>(false),
			visibility: ref<Visibility>('Visible'),

			hostname: ref<string>(),

			settingsOpen: ref<boolean>(false)
		};
	},

	mounted: function () {
		nextTick(async () => {
			this.hostname = await invoke('get_hostname');
			this.version = await getVersion();

			await this.getVisibility();

			if (!await this.store.has(autostartKey)) {
				await this.setAutostart(true);
			} else {
				await this.applyAutostart();
			}

			this.getRealclose();

			// Check permission for notification
			let permissionGranted = await isPermissionGranted();
			if (!permissionGranted) {
				const permission = await requestPermission();
				permissionGranted = permission === 'granted';
			}

			// this.cleanupInterval = setInterval(() => {
			// 	this.toDelete.forEach((itemToDelete) => {
			// 		const now = new Date();
			// 		const timeDifference = now.getTime() - itemToDelete.triggered;

			// 		// Check if at least 30 seconds have passed (30000 milliseconds)
			// 		if (timeDifference >= 30000) this.removeRequest(itemToDelete.id);
			// 	});

			// 	// Clear only elements that have been processed (more than 30s old)
			// 	this.toDelete = this.toDelete.filter((item) => {
			// 		const now = new Date();
			// 		return now.getTime() - item.triggered < 30000;
			// 	});
			// }, 30000);

			this.unlisten.push(
				await listen('rs2js_channelmessage', async (event) => {
					const cm = event.payload as ChannelMessage;
					const idx = this.requests.findIndex((el) => el.id === cm.id);

					if (cm.state === "Disconnected") {
						this.toDelete.push({
							id: cm.id,
							triggered: new Date().getTime()
						});
					}

					if (idx !== -1) {
						const prev = this.requests.at(idx);
						// Update the existing message at index 'idx'
						this.requests.splice(idx, 1, {
							...cm,
							state: cm.state ?? prev!.state,
							meta: cm.meta ?? prev!.meta,
						});
					} else {
						// Push the new message if not found
						this.requests.push(cm);
					}

					return;
				})
			);

			this.unlisten.push(
				await listen('rs2js_endpointinfo', (event) => {
					const ei = event.payload as EndpointInfo;
					const idx = this.endpointsInfo.findIndex((el) => el.id === ei.id);

					if (!ei.present) {
						if (idx !== -1) {
							this.endpointsInfo.splice(idx, 1);
						}

						return;
					}

					if (idx !== -1) {
						this.endpointsInfo.splice(idx, 1, ei);
					} else {
						this.endpointsInfo.push(ei);
					}
				})
			);

			this.unlisten.push(
				await listen('visibility_updated', async () => {
					console.log("Visibility changed");
					await this.getVisibility();
				})
			);

			this.unlisten.push(
				await getCurrent().onFileDropEvent(async (event) => {
					if (event.payload.type === 'hover') {
						this.isDragHovering = true;
					} else if (event.payload.type === 'drop') {
						console.log("Dropped");
						this.isDragHovering = false;
						this.outboundPayload = {
							Files: event.payload.paths
						} as OutboundPayload;
						if (!this.discoveryRunning) await invoke('start_discovery');
						this.discoveryRunning = true;
					} else {
						this.isDragHovering = false;
					}
				})
			);
		});
	},

	unmounted: function() {
		this.unlisten.forEach((el) => el());

		if (this.cleanupInterval && this.cleanupInterval[Symbol.dispose]) {
			this.cleanupInterval[Symbol.dispose]();
		}
	},

	computed: {
		displayedIsEmpty(): boolean {
			return this.displayedItems.length == 0
		},
		displayedItems(): Array<DisplayedItem> {
			const ndisplayed = new Array<DisplayedItem>();

			this.endpointsInfo.forEach((el) => {
				const idx = ndisplayed.findIndex((nel) => el.id == nel.id);
				if (idx !== -1) return;

				ndisplayed.push({
					id: el.id,
					name: el.name ?? 'Unknown',
					deviceType: el.rtype ?? 'Unknown',
					endpoint: true,
				})
			});

			this.requests.filter((el) => stateToDisplay.includes(el.state ?? 'Initial')).forEach((el) => {
				const idx = ndisplayed.findIndex((nel) => el.id == nel.id);
				const elem: DisplayedItem = {
					id: el.id,
					name: el.meta?.source?.name ?? 'Unknown',
					deviceType: el.meta?.source?.device_type ?? 'Unknown',
					endpoint: false,

					state: el.state ?? undefined,
					pin_code: el.meta?.pin_code ?? undefined,
					destination: el.meta?.destination ?? el.meta?.text_payload ?? undefined,
					files: el.meta?.files ?? undefined,
					text_description: el.meta?.text_description ?? undefined,
				};

				if (idx !== -1) {
					ndisplayed.splice(idx, 1, elem);
				} else {
					ndisplayed.push(elem)
				}
			});

			return ndisplayed;
		}
	},

	methods: {
		setAutostart: async function(autostart: boolean) {
			if (autostart) {
				await enable();
			} else {
				await disable();
			}

			await this.store.set(autostartKey, autostart);
			await this.store.save();
			this.autostart = autostart;
		},
		applyAutostart: async function() {
			this.autostart = await this.store.get(autostartKey) ?? false;

			if (this.autostart) {
				await enable();
			} else {
				await disable();
			}
		},
		setRealclose: async function(realclose: boolean) {
			await this.store.set(realcloseKey, realclose);
			await this.store.save();
			this.realclose = realclose;
		},
		getRealclose: async function() {
			this.realclose = await this.store.get(realcloseKey) ?? false;
		},
		setVisibility: async function(visibility: Visibility) {
			await invoke('change_visibility', { message: visibility });
			await this.store.set(visibilityKey, visibilityToNumber[visibility]);
			await this.store.save();
			this.visibility = visibility;
		},
		getVisibility: async function() {
			this.visibility = numberToVisibility[(await this.store.get(visibilityKey) ?? 0) as number];
		},
		invertVisibility: async function() {
			if (this.visibility === 'Temporarily') {
				return;
			}

			if (this.visibility === 'Visible') {
				return this.setVisibility('Invisible');
			}

			return this.setVisibility('Visible');
		},
		clearSending: async function() {
			await invoke('stop_discovery');
			this.outboundPayload = undefined;
			this.discoveryRunning = false;
			this.endpointsInfo = [];
		},
		removeRequest: function(id: string) {
			const idx = this.requests.findIndex((el) => el.id === id);

			if (idx !== -1) {
				this.requests.splice(idx, 1);
			}
		},
		sendInfo: async function(eid: string) {
			if (this.outboundPayload === undefined) return;

			const ei = this.endpointsInfo.find((el) => el.id === eid);
			if (!ei) return;

			const msg: SendInfo = {
				id: ei.id,
				name: ei.name ?? 'Unknown',
				addr: ei.ip + ":" + ei.port,
				ob: this.outboundPayload,
			};

			await invoke('send_payload', { message: msg });
		},
		sendCmd: async function(id: string, action: ChannelAction) {
			const cm: ChannelMessage = {
				id: id,
				direction: 'FrontToLib',
				action: action,
				meta: null,
				state: null,
				rtype: null,
			};
			console.log("js2rs:", cm);

			await invoke('send_to_rs', { message: cm });
		},
		blured: function() {
			(document.activeElement as any).blur();
		},
		openFilePicker: function() {
			dialog.open({
				title: "Select a file to send",
				directory: false,
				multiple: true,
			}).then(async (el) => {
				let elem;
				if (el === null) {
					return;
				}

				console.log("Selected", el);
				if (el instanceof Array) {
					console.log("Is an array");
					elem = el;
				} else {
					console.log("Is not an array");
					elem = [el];
				}


				this.outboundPayload = {
					Files: elem
				} as OutboundPayload;
				if (!this.discoveryRunning) await invoke('start_discovery');
				this.discoveryRunning = true;
			})
		}
	},
}
</script>