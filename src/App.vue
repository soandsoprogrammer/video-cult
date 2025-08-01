<script setup lang="ts">
import Open from './assets/open.svg'
import { open } from '@tauri-apps/plugin-dialog'
import { ref } from "vue";
import { Command } from '@tauri-apps/plugin-shell';
import { openPath } from '@tauri-apps/plugin-opener';

// import { invoke } from "@tauri-apps/api/core";
const filePath = ref<string | null>(null);
const cultFilePath = ref<string | null>(null);
const outputFilePath = ref<string | null>(null);


// const greetMsg = ref("");
// const name = ref("");

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//   greetMsg.value = await invoke("greet", { name: name.value });
// }

async function openFile() {
  const selected = await open({
    multiple: false,
    filters: [
      {
        name: '视频文件',
        extensions: ['mp4', 'avi', 'mkv', 'mov', 'flv']
      }
    ]
  })
  if (selected) {
    filePath.value = selected as string;
  }
}

const openCultFile = async () => {
  const selected = await open({
    multiple: false,
    filters: [
      {
        name: 'Cult文件',
        extensions: ['txt', 'json']
      }
    ]
  })
  if (selected) {
    cultFilePath.value = selected as string;
  }
}

const setOutputFilePath = async () => {
  const selected = await open({
    multiple: false,
    directory: true,
  })
  if (selected) {
    outputFilePath.value = selected as string;
  }
}

const runCommand = async () => {
  console.log('开始处理视频');
  let res = await Command.sidecar('bin/ffmpeg', [
    '-i', filePath.value!,
    '-vf', 'scale=1280:720',
    `${outputFilePath.value}/output.mp4`
  ]).execute();
  console.log('处理结果:', res.stderr);
}

const handleOpenPath = async (path: string) => {
  console.log(path);
  await openPath(path);
}
</script>



<template>
  <div class="con">
    <div data-tauri-drag-region class="main">
      <div v-if="!filePath" class="button" @click="openFile">
        <Open class="icon" />
        <span class="text">打开视频文件</span>
      </div>

      <div v-if="!cultFilePath && filePath" class="button" @click="openCultFile">
        <Open class="icon" />
        <span class="text">打开Cult文件</span>
      </div>
      <div v-if="!outputFilePath && filePath" class="button" @click="setOutputFilePath">
        <Open class="icon" />
        <span class="text">设置输出目录</span>
      </div>
      <div>
        <div class="button" @click="()=>{
          handleOpenPath(outputFilePath!)
        }">
          <span class="text">已选择输出目录: {{ outputFilePath }}</span>
        </div>
        <div class="button">
          <span class="text">已选择Cult文件: {{ cultFilePath }}</span>
        </div>
        <div class="button" @click="openPath(filePath!)">
          <span class="text">已选择视频文件: {{ filePath }}</span>
        </div>
        <div class="button" @click="runCommand">
          <span class="text">点击开始处理视频</span>
        </div>

        <!-- main -->
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.con {
  width: 100vw;
  height: 100vh;
  background-color: transparent;
  padding: 10px;

  .main {
    width: 100%;
    height: 100%;
    background-color: rgba($color: #000000, $alpha: 0.3);
    border-radius: 10px;
    backdrop-filter: blur(20px);
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    position: relative;
    color: #fff;
  }
}

.button {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;

}
</style>
