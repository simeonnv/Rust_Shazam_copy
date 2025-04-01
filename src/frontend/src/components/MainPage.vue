<template>
    <div>
        <button @click="startRecording" :disabled="isRecording || !isSupported">
            Record
        </button>
        <button @click="stopRecording" :disabled="!isRecording">Stop</button>
        <button @click="sendToBackend" :disabled="!audioBlob">
            Send to Backend
        </button>
        <audio controls :src="audioSrc" v-if="audioSrc"></audio>
        <div v-if="error">{{ error.message }}</div>

        <input v-model="url" placeholder="Enter song URL" />
        <button @click="addNewSong">Add Song</button>
    </div>
</template>

<script>
import { ref, computed, onMounted, onUnmounted } from "vue";
import { MediaRecorder, register } from "extendable-media-recorder";
import { connect } from "extendable-media-recorder-wav-encoder";

export default {
    setup() {
        const isSupported = ref(false);
        const isRecording = ref(false);
        const mediaRecorder = ref(null);
        const recordedChunks = ref([]);
        const audioBlob = ref(null);
        const error = ref(null);
        const url = ref(""); // Reactive URL input
        let stream = null;

        onMounted(async () => {
            if (
                !navigator.mediaDevices ||
                !navigator.mediaDevices.getUserMedia
            ) {
                error.value = new Error("MediaDevices API not supported");
                return;
            }

            try {
                await register(await connect());
                isSupported.value = true;
            } catch (err) {
                error.value = err;
            }
        });

        const startRecording = async () => {
            if (isRecording.value || !isSupported.value) return;

            try {
                stream = await navigator.mediaDevices.getUserMedia({
                    audio: true,
                });
                recordedChunks.value = [];
                mediaRecorder.value = new MediaRecorder(stream, {
                    mimeType: "audio/wav",
                });

                mediaRecorder.value.ondataavailable = (event) => {
                    if (event.data.size > 0) {
                        recordedChunks.value.push(event.data);
                    }
                };

                mediaRecorder.value.onstop = () => {
                    audioBlob.value = new Blob(recordedChunks.value, {
                        type: "audio/wav",
                    });
                };

                mediaRecorder.value.start();
                isRecording.value = true;
            } catch (err) {
                error.value = err;
            }
        };

        const stopRecording = () => {
            if (!isRecording.value) return;
            mediaRecorder.value.stop();
            isRecording.value = false;
            if (stream) {
                stream.getTracks().forEach((track) => track.stop());
            }
        };

        const addNewSong = async () => {
            if (!url.value) {
                error.value = new Error("Please enter a URL");
                return;
            }

            try {
                const response = await fetch("http://localhost:8080/songs", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json", // Assuming backend expects JSON
                    },
                    body: JSON.stringify({ url: url.value }), // Stringify the payload
                });

                if (response.ok) {
                    console.log("New Song added successfully");
                    url.value = ""; // Clear the input after success
                } else {
                    error.value = new Error("Failed to add song");
                }
            } catch (err) {
                error.value = err;
            }
        };

        const sendToBackend = async () => {
            if (!audioBlob.value) return;

            try {
                const response = await fetch("http://localhost:8080/songs", {
                    method: "POST",
                    body: audioBlob.value, // Send the raw Blob
                });

                if (response.ok) {
                    console.log("Audio uploaded successfully");
                    audioBlob.value = null; // Clear the blob after success
                } else {
                    error.value = new Error("Upload failed");
                }
            } catch (err) {
                error.value = err;
            }
        };

        const audioSrc = computed(() =>
            audioBlob.value ? URL.createObjectURL(audioBlob.value) : null,
        );

        onUnmounted(() => {
            if (stream) {
                stream.getTracks().forEach((track) => track.stop());
            }
            if (audioSrc.value) {
                URL.revokeObjectURL(audioSrc.value);
            }
        });

        return {
            startRecording,
            stopRecording,
            sendToBackend,
            addNewSong, // Add this to the return object
            isRecording,
            audioBlob,
            audioSrc,
            error,
            isSupported,
            url, // Return the url ref for v-model binding
        };
    },
};
</script>

<style>
button {
    margin: 5px;
}
audio {
    margin-top: 10px;
}
input {
    margin: 5px;
}
</style>
