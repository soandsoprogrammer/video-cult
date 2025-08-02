<script setup lang="ts">
type CultInfo = {
  start_time: string;
  end_time: string;
  filename: string;
  origin_start_time: string;
  origin_end_time: string
}
import Open from './assets/open.svg'
import { open } from '@tauri-apps/plugin-dialog'
import { ref } from "vue";
import { Command } from '@tauri-apps/plugin-shell';
import { openPath } from '@tauri-apps/plugin-opener';

import { invoke } from "@tauri-apps/api/core";

import { VBtn } from 'vuetify/components';
import { path } from '@tauri-apps/api';
import { arrayBufferToBase64 } from './util'
const filePath = ref<string | null>(null);
const cultFilePath = ref<string | null>(null);
const outputFilePath = ref<string | null>(null);
const cultList = ref<CultInfo[]>([]);

const buttonText = ref("开始处理视频");
const loading = ref(false)
const background = ref<string | null>(null);

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
    const dir = await path.appDataDir()
    await Command.sidecar('bin/ffmpeg', ['-y', '-i', selected, '-vframes', '1', '-f', 'image2', `${dir}/bg.png`]).execute();
    invoke('get_first_frame', { filePath: `${dir}/bg.png` })
      .then((data: any) => {
        console.log('获取第一帧成功:');
        background.value = arrayBufferToBase64(data)
      })
      .catch((error) => {
        console.error('获取第一帧失败:', error);
      });
    
    // console.log(dir);
    // console.log(`${}/bg.png`);
    
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
    // 调用 Tauri 命令读取 Cult 文件
    try {
      const result = await invoke('read_file', { filePath: selected });
      cultList.value = result as CultInfo[];
    } catch (error) {
      console.error('读取 Cult 文件失败:', error);
    }
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
  if (loading.value) return
  loading.value = true
  console.log('开始处理');
  for (let i = 0; i < cultList.value.length; i++) {
    const element = cultList.value[i]
    await Command.sidecar('bin/ffmpeg', [
      '-y', '-i', filePath.value!,
      '-ss', element.start_time, '-to', element.end_time,
      '-vn', '-acodec', 'copy',
      `${outputFilePath.value}/${element.filename}.wav`
    ]).execute();
    buttonText.value = `已生成${i}个`
  }
  loading.value = false
  buttonText.value = '重新开始'
  console.log('处理完成')
}

const handleOpenPath = async (path: string) => {
  await openPath(path);
}

</script>



<template>
  <div class="con">
    <div data-tauri-drag-region class="main" :style="{ backgroundImage: `url(${background})` }">
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
      <div v-if="filePath && cultFilePath && outputFilePath" data-tauri-drag-region class="info">
        <div class="paths" style="height: 60vh;" data-tauri-drag-region>

          <div>
            <div @click="openFile">
              <span class="text item">
                <span class="label">已选择视频文件: </span>
                <span class="value">{{ filePath }}</span>
                <v-btn variant="text">
                  更改文件
                </v-btn>
              </span>
            </div>
            <span class="text item">
              <span class="label">已选择Cult文件: </span>
              <span class="value">{{ cultFilePath }}</span>
              <v-btn variant="text" @click="openCultFile">
                更改文件
              </v-btn>
            </span>
          </div>

          <div >
            <span class="text item">
              <span class="label">已选择输出目录: </span>
              <span class="value">{{ outputFilePath }}</span>
              <v-btn variant="text" @click="handleOpenPath(outputFilePath!)">
                打开目录
              </v-btn>
              <v-btn variant="text" @click="setOutputFilePath">
                更改目录
              </v-btn>
            </span>
          </div>
          <div>
            <span>将生成{{ cultList.length }}个音频</span>
            <div style="height: 100px; overflow: auto;">
              <ul>
                <li v-for="(item, index) in cultList" :key="index">
                  <span>{{ index+1 }}. {{ item.origin_start_time }} - {{ item.origin_end_time }} {{ item.filename
                    }}</span>
                </li>
              </ul>
            </div>
          </div>
        </div>

        <div class="bottom flex">
          <v-btn style="color: black;" @click="runCommand">
            {{ buttonText }}
          </v-btn>
        </div>

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
    background-color: rgba($color: #000000, $alpha: 0.8);
    border-radius: 10px;
    backdrop-filter: blur(20px);
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    position: relative;
    color: #fff;
    background-size: contain;
    background-repeat: no-repeat;
    background-position: center;
  }
}

.item {
  display: flex;
  align-items: center;
  margin-bottom: 10px;

  .label {
    width: 120px;
  }

  .value {
    color: rgba($color: #fff, $alpha: 0.65);
    text-decoration: underline;
    flex: 1;
  }
}

.button {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.button2 {}

.info {
  width: 100%;
  height: 100%;
  padding: 10px;
  text-align: left;
}
</style>
