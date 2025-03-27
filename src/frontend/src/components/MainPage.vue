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

        const sendToBackend = async () => {
            if (!audioBlob.value) return;

            const formData = new FormData();
            console.log(audioBlob.value);
            try {
                const response = await fetch("http://localhost:8080", {
                    method: "POST",
                    headers: {
                        "Content-Type": "audio/wav", // Set the correct MIME type
                    },
                    body: audioBlob.value,
                });

                if (response.ok) {
                    console.log("Audio uploaded successfully");
                } else {
                    console.error("Upload failed");
                }
            } catch (err) {
                console.error("Error uploading audio:", err);
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
            isRecording,
            audioBlob,
            audioSrc,
            error,
            isSupported,
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
</style>
