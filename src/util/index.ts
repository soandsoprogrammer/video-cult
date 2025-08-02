/**
 * 将时:分:秒:帧格式转换为时:分:秒（秒为小数）
 * @param timeStr 形如 "01:23:45:12"
 * @param fps 每秒帧数（如25、30等）
 * @returns 形如 "01:23:45.48"
 */
export function timecodeToSecondsString(timeStr: string, fps: number = 24): string {
  const [hh, mm, ss, ff] = timeStr.split(':').map(Number);
  if (
    [hh, mm, ss, ff].some(n => isNaN(n)) ||
    hh < 0 || mm < 0 || mm > 59 || ss < 0 || ss > 59 || ff < 0 || ff >= fps
  ) {
    throw new Error('Invalid timecode or fps');
  }
  const seconds = ss + ff / fps;
  // 保留两位小数
  const secondsStr = seconds.toFixed(2).padStart(5, '0');
  return `${hh.toString().padStart(2, '0')}:${mm.toString().padStart(2, '0')}:${secondsStr}`;
}

export function arrayBufferToBase64(buffer: ArrayBuffer, mimeType = 'image/png') {
  // 将ArrayBuffer转换为Uint8Array
  const uint8Array = new Uint8Array(buffer);
  
  // 创建一个二进制字符串
  let binary = '';
  for (let i = 0; i < uint8Array.length; i++) {
    binary += String.fromCharCode(uint8Array[i]);
  }
  
  // 转换为base64
  const base64String = window.btoa(binary);
  
  // 返回data URI格式
  return `data:${mimeType};base64,${base64String}`;
}