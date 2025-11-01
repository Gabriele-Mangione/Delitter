<script lang="ts">
	import {onMount} from "svelte";
	import {PUBLIC_BACKEND_URL} from "$env/static/public";

	let isCameraActive = false;
	let capturedImage: string | null = null;
	let stream: MediaStream | null = null;
	let videoElement: HTMLVideoElement;
	let canvasElement: HTMLCanvasElement;

	const locationOptions = {
		enableHighAccuracy: true,
		timeout: 10000,
		maximumAge: 10000,
	};

	function success(pos) {
		const crd = pos.coords;

		console.log("Your current position is:");
		console.log(`Latitude : ${crd.latitude}`);
		console.log(`Longitude: ${crd.longitude}`);
		console.log(`More or less ${crd.accuracy} meters.`);

		sendLitterRequest(canvasElement, crd.latitude, crd.longitude)
	}

	function error(err) {
		console.warn(`ERROR(${err.code}): ${err.message}`);
	}

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
			navigator.geolocation.getCurrentPosition(success, error, locationOptions);
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
		const base = PUBLIC_BACKEND_URL;
		const api_url = `${base}/protected/litter`

        const payload = {
			"lat": lat,
			"lng": lng,
			"file": byteArray,
			"type": mimeType,
            "tags": []
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
				console.log('Image successfully sent.');
				const result = await response.json(); // Or response.text()
				console.log('Server response:', result);
				capturedImage = null;
				await startCamera();
			} else {
				console.error(`Upload failed: ${response.status} ${response.statusText}`);
			}
		} catch (error) {
			console.error('Error sending image:', error);
		}
	}

	onMount(() => startCamera())
</script>

<div class="h-center flex flex-col justify-center items-center">

    <div class="flex-row mb-8">
        <video bind:this={videoElement} playsinline
               muted
               autoplay style="display: {isCameraActive ? 'block' : 'none'}"></video>
        <canvas bind:this={canvasElement} style="display: none;"></canvas>

        {#if capturedImage}
            <img src={capturedImage} alt="Captured litter" />
        {/if}
    </div>

    <div class="controls flex-row">
        {#if !isCameraActive && !capturedImage}
            <button class="btn btn-secondary flex justify-center" on:click={startCamera}>Allow Camera</button>
        {:else if isCameraActive}
            <button class="btn btn-secondary" on:click={takePicture}>
                <svg class="size-[1.2em]" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                     stroke="currentColor">
                    <path stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                          d="M6.827 6.175A2.31 2.31 0 0 1 5.186 7.23c-.38.054-.757.112-1.134.175C2.999 7.58 2.25 8.507 2.25 9.574V18a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18V9.574c0-1.067-.75-1.994-1.802-2.169a47.865 47.865 0 0 0-1.134-.175 2.31 2.31 0 0 1-1.64-1.055l-.822-1.316a2.192 2.192 0 0 0-1.736-1.039 48.774 48.774 0 0 0-5.232 0 2.192 2.192 0 0 0-1.736 1.039l-.821 1.316Z"/>
                    <path stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                          d="M16.5 12.75a4.5 4.5 0 1 1-9 0 4.5 4.5 0 0 1 9 0ZM18.75 10.5h.008v.008h-.008V10.5Z"/>
                </svg>
                Take Picture
            </button>
        {/if}

        {#if capturedImage}
            <button class="btn btn-secondary" on:click={() => navigator.geolocation.getCurrentPosition(success, error, locationOptions)}>Retry Upload</button>
        {/if}
    </div>

</div>