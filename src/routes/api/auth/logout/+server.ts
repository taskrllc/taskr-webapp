import { json } from "@sveltejs/kit";
import { destroySession } from "$lib/server/auth";

export async function POST({ cookies }) {
    const sessionId = cookies.get("session_id");
    if (sessionId) {
        await destroySession(sessionId);
        cookies.delete("session_id", { path: "/" });
    }
    return json({ success: true });
}
