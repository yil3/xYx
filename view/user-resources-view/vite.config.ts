import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import vitePluginImp from "vite-plugin-imp";
import { resolve } from "path";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react(),
    vitePluginImp({
      libList: [
        {
          libName: "antd",
          style: (name) => `antd/es/${name}/style`,
        },
      ],
    }),
  ],
  css: {
    preprocessorOptions: {
      less: {
        javascriptEnabled: true,
        modifyVars: {
          "@primary-color": "#4377FE", //设置antd主题色
        },
      },
    },
  },
  server: {
    host: "0.0.0.0",
    port: 3010,
    proxy: {
      "/user_resource": {
        target: "http://127.0.0.1:5010",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/user_resource/, ""),
      },
      "/authen": {
        target: "http://127.0.0.1:5000",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/authen/, ""),
      },
    },
  },
  resolve: {
    alias: {
      "@": resolve(__dirname, "src"),
    },
  },
});
