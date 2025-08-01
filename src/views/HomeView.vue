<template>
  <div class="home-container">
    <div class="welcome-section">
      <h1>欢迎使用 MarkWiki</h1>
      <p>一个简单而强大的个人知识管理工具</p>
    </div>

    <div class="features-section">
      <h2>功能特点</h2>
      <div class="features-grid">
        <div class="feature-card">
          <svg class="feature-icon" xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"></path>
            <path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"></path>
          </svg>
          <h3>多知识库管理</h3>
          <p>创建和管理多个独立的知识库，分类整理您的知识</p>
        </div>
        <div class="feature-card">
          <svg class="feature-icon" xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
            <polyline points="14 2 14 8 20 8"></polyline>
            <line x1="16" x2="8" y1="13" y2="13"></line>
            <line x1="16" x2="8" y1="17" y2="17"></line>
            <polyline points="10 9 9 9 8 9"></polyline>
          </svg>
          <h3>Markdown 编辑</h3>
          <p>使用简洁的 Markdown 语法创建和编辑您的文档</p>
        </div>
        <div class="feature-card">
          <svg class="feature-icon" xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="10"></circle>
            <line x1="2" x2="22" y1="12" y2="12"></line>
            <path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"></path>
          </svg>
          <h3>多设备同步</h3>
          <p>基于 Git 的版本控制，轻松实现多设备间的知识同步</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
</script>

<style scoped>
.home-container {
  flex: 1;
  padding: 2rem;
  max-width: 1200px;
  margin: 0 auto;
  color: #000000;
}

.welcome-section {
  text-align: center;
  margin-bottom: 3rem;
  padding: 2rem;
  background-color: #ffffff;
  border: 1px solid #eee;
  border-radius: 8px;
}

.welcome-section h1 {
  font-size: 2.2rem;
  margin-bottom: 1rem;
  color: #000000;
}

.welcome-section p {
  font-size: 1.1rem;
  color: #333;
  margin-bottom: 2rem;
}

.primary-btn {
  background-color: #f0f0f0;
  color: #000000;
  border: 1px solid #ddd;
  padding: 0.8rem 1.5rem;
  font-size: 1rem;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
}

.primary-btn:hover {
  background-color: #e0e0e0;
  border-color: #ccc;
}

.features-section {
  margin-top: 1rem;
}

.features-section h2 {
  text-align: center;
  margin-bottom: 2rem;
  color: #000000;
  font-size: 1.8rem;
}

.features-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 2rem;
}

.feature-card {
  padding: 2rem;
  background-color: #ffffff;
  border: 1px solid #eee;
  border-radius: 8px;
  text-align: center;
  transition: all 0.2s;
}

.feature-card:hover {
  border-color: #ddd;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.05);
}

.feature-icon {
  margin-bottom: 1rem;
  color: #666;
  width: 40px;
  height: 40px;
}

.feature-card h3 {
  margin-bottom: 1rem;
  color: #000000;
  font-size: 1.3rem;
}

.feature-card p {
  color: #333;
  font-size: 1rem;
  line-height: 1.6;
}

/* 移除深色模式支持 */
</style>