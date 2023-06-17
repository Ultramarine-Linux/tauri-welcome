<template>
  <div class="main">
    <div class="content">
      <h1 class="m-0 mb-4 mt-8">Welcome to Ultramarine</h1>
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
  }

  .background {
    filter: blur(calc(500px / 5));
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
      transform: rotate(-120deg);
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

  @keyframes fade-in2 {
    0% {
      opacity: 0;
    }
    100% {
      opacity: 85%;
    }
  }

  .gradient {
    position: fixed;
    top: 0;
    left: 5%;

    --size: 500px;
    --speed: 10s;
    --easing: cubic-bezier(0.2, 0.2, 0.2, 0.2);

    width: var(--size);
    height: var(--size);
    background-image: -webkit-gradient(
      linear,
      left top,
      left bottom,
      color-stop(0, #913dc6),
      color-stop(1, #3288fa)
    );
    animation: rotate2 var(--speed) var(--easing) infinite,
                fade-in 0.5s ease-in;
    border-radius: 30% 70% 70% 30% / 30% 30% 70% 70%;
  }

  .gradient-2 {
    position: fixed;
    top: 15%;
    right: 5%;

    --size: 500px;
    --speed: 10s;
    --easing: cubic-bezier(0.2, 0.2, 0.2, 0.2);

    width: var(--size);
    height: var(--size);
    background-image: -webkit-gradient(
      linear,
      left top,
      left bottom,
      color-stop(0, #3239ff),
      color-stop(1, #32befa)
    );
    animation: rotate2 var(--speed) var(--easing) infinite,
                fade-in2 0.5s ease-in;
    border-radius: 30% 70% 70% 30% / 30% 30% 70% 70%;
  }
</style>