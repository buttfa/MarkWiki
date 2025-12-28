export const message = {
  success: (text: string) => {
    // 简单实现，可替换为UI库（如Element Plus）的通知组件
    alert(`成功：${text}`);
  },
  error: (text: string) => {
    alert(`错误：${text}`);
  }
};