<script lang="ts">
	let imagePreviewUrl: string | null = null;
	let file: File | null = null;
	let fileInput: HTMLInputElement | null = null;

	function handleFileChange(event: Event) {
		const input = event.target as HTMLInputElement;
		if (!input.files || !input.files[0]) {
			file = null;
			imagePreviewUrl = null;
			return;
		}

		file = input.files[0];
		const reader = new FileReader();
		reader.onload = () => {
			imagePreviewUrl = reader.result as string;
		};
		reader.readAsDataURL(file);
	}

	async function uploadFile() {
		if (!file) return;
		const form = new FormData();
		form.append('file', file);

		try {
			const res = await fetch('/api/upload', {
				method: 'POST',
				body: form
			});
			if (!res.ok) throw new Error('Upload failed');
			// handle success (e.g. show toast, clear state)
			imagePreviewUrl = null;
			file = null;
			if (fileInput) fileInput.value = '';
		} catch (err) {
			console.error(err);
		}
	}

	function removeSelection() {
		file = null;
		imagePreviewUrl = null;
		if (fileInput) fileInput.value = '';
	}
</script>

<div class="min-h-screen bg-base-200">
    <div class="card shadow-md p-6">
        <div class="card-body">
            <h1 class="card-title">Upload Image</h1>

            <div class="form-control">
                <label class="label" for="file-input">
                    <span class="label-text">Choose an image</span>
                </label>
                <input
                        id="file-input"
                        bind:this={fileInput}
                        type="file"
                        accept="image/*"
                        class="file-input file-input-bordered w-full"
                        on:change={handleFileChange}
                />
            </div>

            {#if imagePreviewUrl}
                <div class="flex items-start gap-4 mt-4">
                    <img src={imagePreviewUrl} alt="Preview" class="max-w-xs rounded border"/><br>
                    <div class="flex flex-col gap-2">
                        <button class="btn btn-primary" on:click={uploadFile}>Upload</button>
                        <button class="btn btn-ghost" on:click={removeSelection}>Remove</button>
                    </div>
                </div>
            {:else}
                <p class="text-sm text-muted mt-4">Please select an image to upload.</p>
            {/if}
        </div>
    </div>
    <div class="p-8">
        <h2 class="text-3xl font-bold mb-6 text-center">Photos</h2>
    </div>
</div>