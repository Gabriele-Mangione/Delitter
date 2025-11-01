<script lang="ts">
	import {onDestroy, onMount} from "svelte";
	import {PUBLIC_BACKEND_URL} from "$env/static/public";
	import {browser} from "$app/environment";

	// State variables
	let isCameraActive = false;
	let isWaitingForUpload = false;
	let isWaitingForLocation = false;
	let uploadFailed = false;

	// Camera
	let capturedImage: string | null = null;
	let stream: MediaStream | null = null;
	let videoElement: HTMLVideoElement;
	let canvasElement: HTMLCanvasElement;

	// Geolocation
	let geoWatchId: number | null = null;
	let latitude: number | null = null;
	let longitude: number | null = null;

	async function startCamera() {
		try {
			stream = await navigator.mediaDevices.getUserMedia({
                audio: false,
                video: { facingMode: 'environment' }
			});
			videoElement.srcObject = stream;
			isCameraActive = true;
		} catch (error) {
			console.error('Error starting camera:', error);
		}
	}

	function takePicture() {
		if (videoElement && canvasElement) {
			const context = canvasElement.getContext('2d');
			canvasElement.width = videoElement.videoWidth;
			canvasElement.height = videoElement.videoHeight;
			context?.drawImage(videoElement, 0, 0);
			capturedImage = canvasElement.toDataURL('image/png');
			stopCamera();
            upload();
		}
	}

	function upload() {
		isWaitingForUpload = true;
		if (longitude && latitude) {
			sendLitterRequest(canvasElement, latitude, longitude);
			isWaitingForUpload = false;
		} else {
			console.error("No geolocation data available.");
			isWaitingForLocation = true;

            // Waits for location indefinitely!
            navigator.geolocation.getCurrentPosition((position) =>
            {
				isWaitingForLocation = false;
				const { longitude: lng, latitude: lat } = position.coords;
                sendLitterRequest(canvasElement, lat, lng)
            }, (err) => {
				console.error("Error obtaining geolocation:", err);
				uploadFailed = true;
            });

            isWaitingForUpload = false;
		}
    }

	function stopCamera() {
		if (stream) {
			stream.getTracks().forEach(track => track.stop());
			isCameraActive = false;
		}
	}

	function sendLitterRequest(canvas, lat, lng) {
		canvas.toBlob(async (blob) => {
			if (!blob) {
				console.error("Failed to create blob from canvas.");
				return;
			}

			const reader = new FileReader();

			// When the file is read as an ArrayBuffer
			reader.onloadend = () => {
				const arrayBuffer = reader.result;
				const byteArray = new Uint8Array(arrayBuffer);
				console.log("Image converted to Uint8Array. Size:", byteArray.length, "bytes.");
				uploadAsJson(Array.from(byteArray), blob.type, lat, lng);
			};

			reader.readAsArrayBuffer(blob);

		}, 'image/jpeg', 0.9);
	}

	async function uploadAsJson(byteArray, mimeType, lat, lng) {
        // So we can see the picture shortly after it has been taken.
        await new Promise(resolve => setTimeout(resolve, 1000));

        const base = PUBLIC_BACKEND_URL;
		const api_url = `${base}/protected/litter`

        const payload = {
			"lat": lat,
			"lng": lng,
			"file": byteArray,
			"type": mimeType
		}

		try {
			const response = await fetch(api_url, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
                    'Authorization': `Bearer ${localStorage.getItem('jwt')}`
				},
				body: JSON.stringify(payload)
			});

			if (response.ok) {
				isWaitingForUpload = false;
				console.log('Image successfully sent.');
				const result = await response.json(); // Or response.text()
				console.log('Server response:', result);
				capturedImage = null;
				await startCamera();
			} else {
				console.error(`Upload failed: ${response.status} ${response.statusText}`);
				uploadFailed = true;
			}
		} catch (error) {
			console.error('Error sending image:', error);
		}
	}

	function startLocateWatch() {
		if (!browser || !navigator.geolocation) return;
		if (geoWatchId) navigator.geolocation.clearWatch(geoWatchId);

		geoWatchId = navigator.geolocation.watchPosition(
			(pos) => {
                const coords = pos.coords;
                latitude = coords.latitude;
                longitude = coords.longitude;
                console.log(`Updated position: (${latitude}, ${longitude})`);
            },
			(err) => {
				console.warn(`ERROR(${err.code}): ${err.message}`);
            },
			{ enableHighAccuracy: true, maximumAge: 10000, timeout: 10000 }
		);
	}

	onMount(() => {
		startCamera()
        startLocateWatch()
	})

    onDestroy(() => {
		stopCamera();
	})
</script>

<div class="relative h-full flex flex-col justify-center items-center">
    <canvas
            class="absolute"
            bind:this={canvasElement}
            style="display: none;"
    ></canvas>

    <div class="w-full flex justify-center">
        {#if capturedImage}
            <img src={capturedImage} alt="Captured litter"
                 class="inline-block h-auto max-h-[400px] object-contain rounded-xl border-4 border-black"/>
        {:else}
            <video
                    bind:this={videoElement}
                    playsinline
                    muted
                    autoplay
                    class="inline-block h-auto max-h-[400px] object-contain rounded-xl border-4 border-base-300"
                    style="display: {isCameraActive ? 'block' : 'none'}"
            ></video>
        {/if}
    </div>


    <div class="controls flex-row mt-3">
        {#if !isCameraActive && !capturedImage}
            <button class="btn btn-secondary flex justify-center" on:click={startCamera}>Allow Camera</button>
        {:else if isCameraActive}
            <button class="btn btn-secondary" on:click={takePicture}>
                <svg class="size-[1.4em]" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                     stroke-width="1.5"
                     stroke="currentColor">
                    <path stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                          d="M6.827 6.175A2.31 2.31 0 0 1 5.186 7.23c-.38.054-.757.112-1.134.175C2.999 7.58 2.25 8.507 2.25 9.574V18a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18V9.574c0-1.067-.75-1.994-1.802-2.169a47.865 47.865 0 0 0-1.134-.175 2.31 2.31 0 0 1-1.64-1.055l-.822-1.316a2.192 2.192 0 0 0-1.736-1.039 48.774 48.774 0 0 0-5.232 0 2.192 2.192 0 0 0-1.736 1.039l-.821 1.316Z"/>
                    <path stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                          d="M16.5 12.75a4.5 4.5 0 1 1-9 0 4.5 4.5 0 0 1 9 0ZM18.75 10.5h.008v.008h-.008V10.5Z"/>
                </svg>
                Take Picture
            </button>
        {/if}

        {#if isWaitingForUpload}
            <span class="align-[-8px] align-middle loading loading-ring loading-lg"/>
            <span class="">Waiting for upload...</span>
        {/if}

        {#if isWaitingForLocation}
            <span class="align-[-8px] loading loading-ring loading-lg"/>
            <span class="">Waiting for location...</span>
        {/if}

        {#if uploadFailed}
            <span class="ml-4">Upload failed</span>
            <button class="btn btn-secondary" on:click={() => upload()}>Retry Upload</button>
        {/if}
    </div>

</div>