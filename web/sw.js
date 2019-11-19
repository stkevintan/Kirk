// importScripts('/workbox-v4.3.1/workbox-sw.js')
// workbox.setConfig({ modulePathPrefix: '/workbox-v4.3.1' })

// importScripts('/precache-manifest.1ab39011a8a55317c0504341966c3f8e.js')

workbox.core.skipWaiting()
workbox.core.clientsClaim()

/**
 * The workboxSW.precacheAndRoute() method efficiently caches and responds to
 * requests for URLs in the manifest.
 * See https://goo.gl/S9QRab
 */
self.__precacheManifest = [].concat(self.__precacheManifest || [])
workbox.precaching.precacheAndRoute(self.__precacheManifest, {})

workbox.routing.registerNavigationRoute('/', {
  blacklist: [/\.(css|js|wasm|png|jpg|svg)$/, /\/api\./, /sockjs-node/]
})

workbox.routing.registerRoute(
  /^https:\/\/api\.github\.com\//,
  new workbox.strategies.StaleWhileRevalidate(),
  'GET'
)
workbox.routing.registerRoute(
  /^https:\/\/(cdnjs\.cloudflare\.com|at\.alicdn\.com)\/$/,
  new workbox.strategies.CacheFirst(),
  'GET'
)
workbox.routing.registerRoute(
  /\.html$/,
  new workbox.strategies.NetworkFirst(),
  'GET'
)
