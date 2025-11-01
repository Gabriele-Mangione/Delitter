<script lang="ts">
	import {onMount} from "svelte";

	let isCameraActive = false;
	let capturedImage: string | null = null;
	let stream: MediaStream | null = null;
	let videoElement: HTMLVideoElement;
	let canvasElement: HTMLCanvasElement;

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
		}
	}

	function stopCamera() {
		if (stream) {
			stream.getTracks().forEach(track => track.stop());
			isCameraActive = false;
		}
	}

	function downloadPicture() {
		if (capturedImage) {
			const a = document.createElement('a');
			a.href = capturedImage;
			a.download = `litter-${Date.now()}.png`;
			a.click();
		}
	}

	onMount(() => startCamera())
</script>

<div class="min-h-screen bg-base-200">
    <div class="hero">
        <div class="hero-content">
            <div class="max-w-md">
                <h1 class="text-5xl font-bold">Take litter picture</h1>
            </div>
        </div>
    </div>

    <div class="flex flex-col justify-center items-center">

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
                <button class="btn btn-secondary" on:click={takePicture}>Take Picture</button>
            {/if}

            {#if capturedImage}
                <button on:click={downloadPicture}>Download Picture</button>
                <button on:click={() => { capturedImage = null; startCamera(); }}>Take Another</button>
            {/if}
        </div>

    </div>
</div>