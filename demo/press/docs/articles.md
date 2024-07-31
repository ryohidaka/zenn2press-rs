<script setup>
import { data as articles } from './.vitepress/articles.data.ts'
</script>


<ul>
  <li v-for="(article, index) in articles" :key="index">
    <a v-if="article.frontmatter.title" :href="article.url">
      {{ article.frontmatter.title }}
    </a>
  </li>
</ul>
