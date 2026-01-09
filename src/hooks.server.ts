import type { Handle } from "@sveltejs/kit";
import { verifySession } from "$lib/server/auth";

export const handle: Handle = async ({ event, resolve }) => {
  const sessionCookie = event.cookies.get("session_id");
  if (sessionCookie) {
    const user = await verifySession(sessionCookie);
    if (user) event.locals.user = user;
  }
  return resolve(event);
};
