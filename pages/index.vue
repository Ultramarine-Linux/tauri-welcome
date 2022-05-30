<template>
    <div class="h-screen w-full bg-gray-800">
        <h1 class="text-red-500">Base app with nuxt 3, tailwind and tauri</h1>
        <button> Button styled using Fluent GTK CSS </button>
    </div>
</template>

<style scoped>
button {
  min-height: 24px;
  min-width: 16px;
  padding: 4px 8px;
  border-radius: 2px;
  font-weight: 500;
  border: 1px solid rgba(255, 255, 255, 0.12);
  transition: all 75ms cubic-bezier(0, 0, 0.2, 1), border 300ms cubic-bezier(0, 0, 0.2, 1), box-shadow 300ms cubic-bezier(0, 0, 0.2, 1);
  outline: none;
  box-shadow: inset 0 0 0 1px transparent, 0 0 0 2px transparent;
  background-color: #2B2B2B;
  color: white;
  border-color: rgba(255, 255, 255, 0.08);
  border-top-color: rgba(255, 255, 255, 0.12);
}

button:focus, button:checked:focus {
  box-shadow: inset 0 0 0 1px transparent, 0 0 0 2px rgba(255, 255, 255, 0.7);
  border-color: transparent;
  background-clip: padding-box;
}

button:hover {
  box-shadow: inset 0 0 0 1px transparent, 0 0 0 2px transparent;
  border-color: rgba(255, 255, 255, 0.3);
}

button:active {
  animation: none;
  transition: all 75ms cubic-bezier(0, 0, 0.2, 1), border 300ms cubic-bezier(0, 0, 0.2, 1), box-shadow 300ms cubic-bezier(0, 0, 0.2, 1);
  box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.3), 0 0 0 2px transparent;
}

button:disabled {
  box-shadow: none;
  background-color: #303030;
  color: rgba(255, 255, 255, 0.5);
  border-color: rgba(255, 255, 255, 0.04);
}

button:checked {
  box-shadow: inset 0 0 0 1px transparent, 0 0 0 2px transparent;
  background-clip: border-box;
  background-color: #3281ea;
  color: white;
  border-color: #498fec;
  border-bottom-color: #1b73e8;
  transition: all 75ms cubic-bezier(0, 0, 0.2, 1), background-color 300ms cubic-bezier(0, 0, 0.2, 1);
}

button:checked:hover {
  box-shadow: inset 0 0 0 9999px transparent;
}

button:checked:disabled {
  box-shadow: none;
  background-color: rgba(50, 129, 234, 0.35);
  border-color: rgba(50, 129, 234, 0.35);
  background-clip: padding-box;
  color: rgba(255, 255, 255, 0.5);
}

button:drop(active) {
  box-shadow: inset 0 0 0 1px transparent, 0 0 0 2px transparent;
  border-color: rgba(255, 255, 255, 0.3);
}
</style>