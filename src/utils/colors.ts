// 标签颜色工具

// 预定义调色板（柔和色调）
const PALETTE = [
  { bg: '#e8f5e9', text: '#2e7d32' }, // 绿
  { bg: '#e3f2fd', text: '#1565c0' }, // 蓝
  { bg: '#fff3e0', text: '#e65100' }, // 橙
  { bg: '#fce4ec', text: '#c62828' }, // 红
  { bg: '#f3e5f5', text: '#7b1fa2' }, // 紫
  { bg: '#e0f7fa', text: '#00838f' }, // 青
  { bg: '#fff9c4', text: '#f57f17' }, // 黄
  { bg: '#efebe9', text: '#4e342e' }, // 棕
  { bg: '#e8eaf6', text: '#283593' }, // 靛蓝
  { bg: '#f1f8e9', text: '#558b2f' }, // 浅绿
  { bg: '#fbe9e7', text: '#bf360c' }, // 深橙
  { bg: '#e1f5fe', text: '#0277bd' }, // 浅蓝
]

// 简单字符串哈希
function hashString(str: string): number {
  let hash = 0
  for (let i = 0; i < str.length; i++) {
    const char = str.charCodeAt(i)
    hash = ((hash << 5) - hash) + char
    hash |= 0 // 转为 32 位整数
  }
  return Math.abs(hash)
}

/** 根据标签名获取颜色 */
export function getTagColor(tag: string): { bg: string; text: string } {
  const index = hashString(tag) % PALETTE.length
  return PALETTE[index]
}
