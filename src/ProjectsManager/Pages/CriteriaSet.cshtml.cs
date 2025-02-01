using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.RazorPages;
using ProjectsManager.Models;
using ProjectsManager.Services;

namespace ProjectsManager.Pages;

public class CriteriaModel(ProjectsApiService apiService) : PageModel
{
    private readonly ProjectsApiService _apiService = apiService;

    [BindProperty]
    public Criteria Criteria { get; set; } = new();

    public void OnGet()
    {
    }

    public async Task<IActionResult> OnPostAsync()
    {
        if (!ModelState.IsValid)
        {
            return Page();
        }

        bool success = await _apiService.CreateCriteriaAsync(Criteria);
        if (success)
        {
            TempData["SuccessMessage"] = "Criteria set successfully created!";
            return RedirectToPage("/CriteriaSet");
        }
        else
        {
            ModelState.AddModelError(string.Empty, "Failed to create criteria.");
            return Page();
        }
    }
}
