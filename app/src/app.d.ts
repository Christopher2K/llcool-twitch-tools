/// <reference types="@sveltejs/kit" />

// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare namespace App {
  interface Locals {
    user?: import('@app/models').User,
    isBotUser?: boolean
  }

  // interface Platform {}
  // interface Session {}
  // interface Stuff {}
}
