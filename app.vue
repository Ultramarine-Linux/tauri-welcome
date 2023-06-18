<template>
  <div class="main">
    <div class="content">
      <div class="ultramarine-logo">
        <svg id="svg5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 224 224"><g id="layer1"><g id="surface85753"><path id="path1984" class="um-cls-1" d="M112,0A112,112,0,1,1,0,112,112,112,0,0,1,112,0" transform="translate(0 0)"/><path id="path1986" class="um-cls-2" d="M206.49,147.8c-16.67-3.84-33-9.53-44.13-16C131.85,114,121.29,75.15,78.26,89.6c34,39.65-23,52-61.64,55.76a101,101,0,0,0,189.87,2.44" transform="translate(0 0)"/></g></g></svg>
      </div>
      <h1 class="header">Welcome to Ultramarine Linux</h1>
      <span class="header-span">We'll guide you through the most important steps to make the system yours.</span>
      <Button label="Get Started"></Button>
    </div>
    
    <div class="background">
      <div class="gradient"></div>
      <div class="gradient-2"></div>
    </div>
  </div>
</template>

<style>
  body {
    margin: 0;
    padding: 0;
    font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  }

  .main {
    background-color: #0f0f11;
    height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .content {
    z-index: 10;
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-top: 2.5rem;
    animation: fade-in 2s ease-in-out;
  }

  .um-cls-1{fill:#00078f;fill-rule:evenodd;}
  .um-cls-2{fill:#fdfdff;}

  .header {
    margin: 0;
    margin-top: 1rem;
    margin-bottom: 0.5rem;
    font-size: 2.5em;
  }

  .header-span {
    letter-spacing: 0.01em;
    margin-bottom: 2rem;
    color: rgb(240, 240, 240);
  }

  .ultramarine-logo {
    width: 120px;
    height: auto;
  }

  .background {
    opacity: 0;
    filter: blur(calc(100px));
    animation: fade-in 1.25s ease-in-out;
    animation-delay: 0.75s;
    animation-fill-mode: forwards;
  }

  @keyframes rotate {
    0% {
      opacity: 60%;
      transform: rotate(0deg);
    }
    50% {
      transform: rotate(120deg);
      opacity: 100%;
    }
    100% {
      transform: rotate(0deg);
      opacity: 60%;
    }
  }

  @keyframes rotate2 {
    0% {
      opacity: 85%;
      transform: rotate(0deg);
    }
    40% {
      transform: rotate(360deg);
      opacity: 60%;
    }
    100% {
      transform: rotate(0deg);
      opacity: 85%;
    }
  }

  @keyframes fade-in {
    0% {
      opacity: 0;
    }
    100% {
      opacity: 100%;
    }
  }

  .gradient {
    position: fixed;
    top: 0;
    left: 5%;

    --size: 500px;
    --speed: 5s;
    --easing: cubic-bezier(0.2, 0.2, 0.2, 0.2);

    width: var(--size);
    height: var(--size);
    background-image: -webkit-gradient(
      linear,
      left top,
      left bottom,
      color-stop(0, #b300ff),
      color-stop(1, #3257fa)
    );
    animation: rotate var(--speed) var(--easing) infinite alternate-reverse;
    border-radius: 30% 70% 70% 30% / 30% 30% 70% 70%;
  }

  .gradient-2 {
    position: fixed;
    top: 15%;
    right: 5%;

    --size: 500px;
    --speed: 8s;
    --easing: cubic-bezier(0.1, 0.2, 0.2, 0.1);

    width: var(--size);
    height: var(--size);
    background-image: -webkit-gradient(
      linear,
      left top,
      left bottom,
      color-stop(0, #2227b7),
      color-stop(1, #32befa)
    );
    animation: rotate2 var(--speed) var(--easing) infinite alternate;
    border-radius: 30% 70% 70% 30% / 30% 30% 70% 70%;
  }
</style>