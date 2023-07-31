// @ts-check
//
// The line above enables type checking for this file. Various IDEs interpret
// the @ts-check directive. It will give you helpful autocompletion when
// implementing this exercise.

/**
 * Removes duplicate tracks from a playlist.
 *
 * @param {string[]} playlist
 * @returns {string[]} new playlist with unique entries
 */
export function removeDuplicates(playlist) {
  let deduplicated = new Set();

  for (const song of playlist) {
    deduplicated.add(song);
  }

  return Array.from(deduplicated);
}

/**
 * Checks whether a playlist includes a track.
 *
 * @param {string[]} playlist
 * @param {string} track
 * @returns {boolean} whether the track is in the playlist
 */
export function hasTrack(playlist, track) {
  let deduplicated = new Set();

  for (const song of playlist) {
    deduplicated.add(song);
  }

  const length = deduplicated.size;
  deduplicated.add(track);

  return length == deduplicated.size;
}

/**
 * Adds a track to a playlist.
 *
 * @param {string[]} playlist
 * @param {string} track
 * @returns {string[]} new playlist
 */
export function addTrack(playlist, track) {
  if (!hasTrack(playlist, track)) {
    playlist.push(track);
  }

  return playlist;
}

/**
 * Deletes a track from a playlist.
 *
 * @param {string[]} playlist
 * @param {string} track
 * @returns {string[]} new playlist
 */
export function deleteTrack(playlist, track) {
  let deduplicated = new Set();

  for (const song of playlist) {
    deduplicated.add(song);
  }

  deduplicated.delete(track);
  return Array.from(deduplicated);
}

/**
 * Lists the unique artists in a playlist.
 *
 * @param {string[]} playlist
 * @returns {string[]} list of artists
 */
export function listArtists(playlist) {
  let deduplicated = new Set();

  for (const song of playlist) {
    const artist = song.split(" - ")[1];
    deduplicated.add(artist);
  }
  return Array.from(deduplicated);
}
