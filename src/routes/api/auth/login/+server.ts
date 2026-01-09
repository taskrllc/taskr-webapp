import { json, error } from "@sveltejs/kit";
import { authenticate, createSession } from "$lib/server/auth";

export async function POST({ request, cookies }) {
  const { email, password } = await request.json();
  const user = await authenticate(email, password);

  if (!user) throw error(401, "Invalid credentials");

  const session = await createSession(user.id);
  cookies.set("session_id", session.id, {
    path: "/",
    httpOnly: true,
    sameSite: "lax",
    secure: process.env.NODE_ENV === "production",
  });

  return json({ user });
}
