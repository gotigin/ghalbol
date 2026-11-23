// Firebase web app for ghalbol.com — Analytics + Auth.
import { initializeApp } from "firebase/app";
import { getAnalytics } from "firebase/analytics";
import {
  getAuth,
  GoogleAuthProvider,
  signInWithPopup,
  signInWithEmailAndPassword,
  createUserWithEmailAndPassword,
} from "firebase/auth";

const firebaseConfig = {
  apiKey: "AIzaSyBd34wBvzIxqunE_VBFTV91CR9Ijtmao7I",
  authDomain: "ghalbol.com",
  projectId: "ghalbol",
  storageBucket: "ghalbol.firebasestorage.app",
  messagingSenderId: "680984321038",
  appId: "1:680984321038:web:e8516762c34163c1d7705f",
  measurementId: "G-4JG0KBTYNY",
};

const app = initializeApp(firebaseConfig);
const auth = getAuth(app);

let analytics = null;
try {
  analytics = getAnalytics(app);
} catch {
  // Analytics may fail offline / in some local contexts; Auth still works.
}

if (typeof window !== "undefined") {
  window.__SITE_AUTH__ = auth;
  window.__SITE_APP__ = app;
  window.__SITE_PRODUCT_NAME__ = "Ghal Bol";
  window.__SITE_ACCOUNT_HREF__ = window.__SITE_ACCOUNT_HREF__ || "#";
  window.__SITE_SIGN_IN_GOOGLE__ = () =>
    signInWithPopup(auth, new GoogleAuthProvider());
  window.__SITE_SIGN_IN_EMAIL__ = (email, password) =>
    signInWithEmailAndPassword(auth, email, password);
  window.__SITE_REGISTER_EMAIL__ = (email, password) =>
    createUserWithEmailAndPassword(auth, email, password);
}

export { app, auth, analytics };
