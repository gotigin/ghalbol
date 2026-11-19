// Firebase web app (Analytics) for ghalbol.com — bundled into static/js/firebase-init.js.
// Same shape as kumkaj/khaynjo and wadhu site clients.
import { initializeApp } from "firebase/app";
import { getAnalytics } from "firebase/analytics";

const firebaseConfig = {
  apiKey: "AIzaSyBd34wBvzIxqunE_VBFTV91CR9Ijtmao7I",
  authDomain: "ghalbol.firebaseapp.com",
  projectId: "ghalbol",
  storageBucket: "ghalbol.firebasestorage.app",
  messagingSenderId: "680984321038",
  appId: "1:680984321038:web:e8516762c34163c1d7705f",
  measurementId: "G-4JG0KBTYNY",
};

const app = initializeApp(firebaseConfig);

let analytics = null;
try {
  analytics = getAnalytics(app);
} catch {
  // Analytics may fail offline / in some local contexts; the site still works.
}

export { app, analytics };
