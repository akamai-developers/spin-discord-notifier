const notifyButton = document.getElementById("notify-btn") as HTMLButtonElement;
const successMsg = document.getElementById("success-msg") as HTMLDivElement;

notifyButton.addEventListener("click", async () => {
  notifyButton.disabled = true;

  try {
    await fetch("/api/notifications", { method: "POST" });
  } catch (error) {
    console.log("Error sending notification:", error);
  }
  notifyButton.classList.add("hidden");
  successMsg.classList.remove("hidden");
  successMsg.classList.add("flex");
});
